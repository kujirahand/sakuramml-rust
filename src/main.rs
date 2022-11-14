/// Command line tool

use std::fs::{File, read_to_string};
use std::io::{Write};

use sakuramml::sakura_version::SAKURA_VERSION;
use sakuramml::lexer::lex;
use sakuramml::song::Song;
use sakuramml::midi::{generate, dump_midi};
use sakuramml::runner::exec;

/// show usage
fn usage() {
    println!("=== sakuramml ver.{} ===\n{}{}{}{}{}{}{}",
        SAKURA_VERSION,
        "USAGE:\n",
        "  sakuramml (mmlfile) (midifile)\n",
        "OPTIONS:\n",
        "  -d, --debug    Debug mode\n",
        "  -e, --exec     Compile (MML)\n",
        "  -h, --help     Show help\n",
        "  -v, --version  Show version\n",
    );
}

/// show sakura version
fn version() {
    println!("{}", SAKURA_VERSION);
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
        else if arg == "--debug" || arg == "-d" || arg == "debug" || arg == "d" {
            song.debug = true;
        }
        else if arg == "--eval" || arg == "-e" || arg == "eval" || arg == "e" {
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
    let tokens = lex(&mut song, &src, 0);
    // println!("lex= {:?}", tokens); 
    exec(&mut song, &tokens);
    // println!("song= {:?}", song);
    save_to_file(&mut song, &outfile);
    println!("{}\nok.", song.logs.join("\n").trim());
}

/// save song to file
fn save_to_file(song: &mut Song, path: &str) {
    let mut file = File::create(path).unwrap();
    let buf = generate(song);
    if song.debug {
        dump_midi(&buf);
    }
    file.write(buf.as_ref()).unwrap();
    file.flush().unwrap();
}
