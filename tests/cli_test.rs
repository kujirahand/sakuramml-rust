use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use sakuramml::song::SAKURA_DEFAULT_MAX_EVENT_BYTES;

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
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("USAGE:"));
    assert!(help_stdout.contains(&format!(
        "--max-event-bytes N  Set MIDI event data limit (default: {})",
        SAKURA_DEFAULT_MAX_EVENT_BYTES,
    )));
    if option_env!("BUILD_NUMBER").unwrap_or("").trim().is_empty() {
        assert!(!help_stdout.contains("(build:"));
    }

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
fn dump_outputs_midi_channels() {
    let dir = TestDir::new("dump-channel");
    let compile = run(
        &["--eval", "TR(1) o4 CH(1) c CH(2) d TR(2) CH(10) CC(1,100)"],
        &dir,
    );
    assert!(
        compile.status.success(),
        "{}",
        String::from_utf8_lossy(&compile.stderr)
    );

    let dump = run(&["--dump", "eval.mid"], &dir);
    assert!(
        dump.status.success(),
        "{}",
        String::from_utf8_lossy(&dump.stderr)
    );
    let stdout = String::from_utf8_lossy(&dump.stdout);
    let lines: Vec<_> = stdout.lines().collect();
    assert!(lines.contains(&"TR(0)"), "{stdout}");
    assert!(lines.contains(&"TR(1) CH(1)"), "{stdout}");
    assert!(stdout.contains("CH(2) NoteOn($32,$64)"), "{stdout}");
    assert!(lines.contains(&"TR(2) CH(10)"), "{stdout}");
    assert!(
        !lines.iter().any(|line| line.starts_with("TR(3)")),
        "{stdout}"
    );
    assert_eq!(stdout.matches("CH(1)").count(), 1, "{stdout}");
    assert_eq!(stdout.matches("CH(2)").count(), 1, "{stdout}");
    assert_eq!(stdout.matches("CH(10)").count(), 1, "{stdout}");
}

#[test]
fn dump_uses_program_change_as_initial_channel() {
    let dir = TestDir::new("dump-program-change-channel");
    let compile = run(&["--eval", "TR(1) CH(4) Voice(2)"], &dir);
    assert!(
        compile.status.success(),
        "{}",
        String::from_utf8_lossy(&compile.stderr)
    );

    let dump = run(&["--dump", "eval.mid"], &dir);
    assert!(
        dump.status.success(),
        "{}",
        String::from_utf8_lossy(&dump.stderr)
    );
    let stdout = String::from_utf8_lossy(&dump.stdout);
    let lines: Vec<_> = stdout.lines().collect();
    assert!(lines.contains(&"TR(0)"), "{stdout}");
    assert!(lines.contains(&"TR(1) CH(4)"), "{stdout}");
    assert!(stdout.contains("Voice(2)"), "{stdout}");
    assert_eq!(stdout.matches("CH(4)").count(), 1, "{stdout}");
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

#[test]
fn max_event_bytes_stops_compilation_and_writes_a_partial_file() {
    let dir = TestDir::new("event-limit");
    let output = run(
        &["--max-event-bytes", "64", "--eval", "[1000000 y1,64]"],
        &dir,
    );
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("MIDI event data exceeds max_event_bytes (64)"));
    assert!(fs::read(dir.0.join("eval.mid"))
        .unwrap()
        .starts_with(b"MThd"));
}

#[test]
fn max_event_bytes_accepts_a_custom_cli_limit() {
    let dir = TestDir::new("event-limit-custom");
    let output = run(&["--max-event-bytes", "16", "--eval", "y1,64 y2,64"], &dir);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(fs::read(dir.0.join("eval.mid"))
        .unwrap()
        .starts_with(b"MThd"));
}
