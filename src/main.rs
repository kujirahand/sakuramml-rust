use std::fs::{File, read_to_string};
use std::io::{Write};

use sakuramml::token::lex;
use sakuramml::song::{exec, Song};
use sakuramml::midi::generate;

fn usage() {
    println!("=== sakuramml ver.{} ===\n{}{}{}{}{}{}{}",
        sakuramml::sakura_version::version_str(),
        "USAGE:\n",
        "  sakuramml (mmlfile) (midifile)\n",
        "OPTIONS:\n",
        "  -d, --debug    Debug mode\n",
        "  -e, --exec     Compile (MML)\n",
        "  -h, --help     Show help\n",
        "  -v, --version  Show version\n",
    );
}
fn version() {
    println!("{}", sakuramml::sakura_version::version_str());
}
fn main() {
    let mut song = Song::new();
    let args: Vec<String> = std::env::args().collect();
    let mut filename = String::new();
    let mut outfile = String::new();
    let mut eval_mml = String::new();
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" || arg == "-h" {
            usage();
            return;
        }
        else if arg == "--version" || arg == "-v" || arg == "version" {
            version();
            return;
        }
        else if arg == "--debug" || arg == "-d" {
            song.debug = true;
        }
        else if arg == "--eval" || arg == "-e" || arg == "eval" {
            i += 1;
            eval_mml = if i < args.len() { String::from(&args[i]) } else { String::new() };
            outfile = String::from("eval.mid");
        }
        else if filename == "" {
            filename = arg.clone();
        }
        else if outfile == "" {
            outfile = arg.clone();
        }
        i += 1;
    }
    if filename == "" && eval_mml == "" {
        usage();
        return;
    }
    if outfile == "" {
        outfile = filename.replace(".mml", ".mid");
    }

    // read file
    let mut src: String;
    if eval_mml != "" {
        src = eval_mml;
    } else {
        src = match read_to_string(filename.clone()) {
            Ok(s) => s,
            Err(e) => {
                println!("[ERROR](0): File not found : {}", filename);
                panic!("{:?}", e);
            }
        };
    }
    // sutoton
    src = sakuramml::sutoton::convert(&src);
    // println!("{}", src);
    let tokens = lex(&mut song, &src);
    // println!("lex= {:?}", tokens); 
    exec(&mut song, &tokens);
    // println!("song= {:?}", song);
    save_to_file(&song, &outfile);
    println!("ok.");
}

fn save_to_file(song: &Song, path: &str) {
    let mut file = File::create(path).unwrap();
    let buf = generate(song);
    file.write(buf.as_ref()).unwrap();
    file.flush().unwrap();
}
