use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

struct TestDir(PathBuf);

impl TestDir {
    fn new(name: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("現在時刻を取得できません")
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("sakuramml-{name}-{}-{unique}", std::process::id()));
        fs::create_dir(&path).expect("テスト用ディレクトリを作成できません");
        Self(path)
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run(args: &[&str], current_dir: &TestDir) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sakuramml"))
        .args(args)
        .current_dir(&current_dir.0)
        .output()
        .expect("sakurammlを実行できません")
}

#[test]
fn help_and_version_succeed() {
    let dir = TestDir::new("help");
    let help = run(&["--help"], &dir);
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("USAGE:"));

    let version = run(&["--version"], &dir);
    assert!(version.status.success());
    assert!(!String::from_utf8_lossy(&version.stdout).trim().is_empty());
}

#[test]
fn eval_creates_a_midi_file() {
    let dir = TestDir::new("eval");
    let output = run(&["--eval", "o4c"], &dir);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("ok."));
    assert!(fs::read(dir.0.join("eval.mid"))
        .unwrap()
        .starts_with(b"MThd"));
}

#[test]
fn input_file_uses_mid_as_the_default_extension() {
    let dir = TestDir::new("file");
    fs::write(dir.0.join("song.mml"), "o4c").unwrap();

    let output = run(&["song.mml"], &dir);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(fs::read(dir.0.join("song.mid"))
        .unwrap()
        .starts_with(b"MThd"));
}

#[test]
fn missing_input_file_reports_an_error() {
    let dir = TestDir::new("missing");
    let output = run(&["missing.mml"], &dir);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("File not found"));
    assert!(!dir.0.join("missing.mid").exists());
}

#[test]
fn eval_without_source_reports_an_error() {
    let dir = TestDir::new("eval-missing");
    let output = run(&["--eval"], &dir);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires MML text"));
    assert!(!dir.0.join("eval.mid").exists());
}
