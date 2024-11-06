//! Command line interface

use std::fs::{self, File, read_to_string};
use std::io::{Write, Read};

use sakuramml::sakura_version::SAKURA_VERSION;
use sakuramml::lexer::lex;
use sakuramml::song::{Song, SAKURA_DEFAULT_RANDOM_SEED};
use sakuramml::midi::{generate, dump_midi};
use sakuramml::runner::exec;

// for randomize
use std::time::SystemTime;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::thread;
fn thread_id_to_u64() -> u64 {
    let thread_id = thread::current().id();
    let mut hasher = DefaultHasher::new();
    thread_id.hash(&mut hasher);
    hasher.finish()
}
fn time_to_u64() -> u64 {
    let now = SystemTime::now();
    let duration = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");
    duration.as_millis() as u64  // または as_secs() などを使用
}

/// show usage
fn usage() {
    println!("=== sakuramml ver.{} ===\n{}{}{}{}{}{}{}{}",
        SAKURA_VERSION,
        "USAGE:\n",
        "  sakuramml (mmlfile) (midifile)\n",
        "OPTIONS:\n",
        "  -d, --debug    Debug mode\n",
        "  -e, --exec     Compile (MML)\n",
        "  -h, --help     Show help\n",
        "  -v, --version  Show version\n",
        "  -m, --dump,    Dump midi file\n",
    );
}

/// show sakura version
fn version() {
    println!("{}", SAKURA_VERSION);
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut filename = String::new();
    let mut outfile = String::new();
    let mut eval_mml = String::new();
    let mut mode = String::from("mml2mid");
    let mut debug = false;
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" || arg == "-h" || arg == "help" {
            usage();
            return;
        }
        else if arg == "--version" || arg == "-v" || arg == "version" {
            version();
            return;
        }
        else if arg == "--debug" || arg == "-d" || arg == "debug" || arg == "d" {
            debug = true;
        }
        else if arg == "--eval" || arg == "-e" || arg == "eval" || arg == "e" {
            i += 1;
            eval_mml = if i < args.len() { String::from(&args[i]) } else { String::new() };
            outfile = String::from("eval.mid");
        }
        else if arg == "--dump" || arg == "dump" || arg == "-m" {
            mode = String::from("dump");
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
        outfile.push_str(&filename);
        outfile.push_str(".mid");
        outfile = outfile.replace(".mml.mid", ".mid");
    }
    // dump mode
    if mode == "dump" {
        match fs::File::open(filename.clone()) {
            Ok(mut f) => {
                let mut buf: Vec<u8> = vec![];
                f.read_to_end(&mut buf).unwrap();
                dump_midi(&buf, true);
                return;
            },
            Err(_e) => {
                println!("[ERROR](0): File not found : {}", filename);
                return;
            }
        }
    }
    // read file
    let src: String;
    if eval_mml != "" {
        src = eval_mml;
    } else {
        src = match read_to_string(filename.clone()) {
            Ok(s) => s,
            Err(_e) => {
                println!("[ERROR](0): File not found : {}", filename);
                return;
            }
        };
    }
    // --- compile mml to midi ---
    compile_to_midi(&src, &outfile, debug);
}

fn compile_to_midi(src: &str, midifile: &str, debug: bool) {
    let mut song = Song::new();
    song.debug = debug;
    song.rand_seed = SAKURA_DEFAULT_RANDOM_SEED ^ (time_to_u64() ^ thread_id_to_u64()) as u32;
    // sutoton
    let src = sakuramml::sutoton::convert(&src);
    // println!("{}", src);
    let tokens = lex(&mut song, &src, 0);
    if debug {
        let tokens_str = sakuramml::token::tokens_to_debug_str(&tokens, 0);
        println!("[PARSER]\n{}", tokens_str);
        println!("[RUNNER]");
    }
    // println!("lex= {:?}", tokens);
    exec(&mut song, &tokens);
    // println!("song= {:?}", song);
    save_to_file(&mut song, &midifile);
    println!("{}\nok.", song.get_logs_str().trim());
}

/// save song to file
fn save_to_file(song: &mut Song, path: &str) {
    let mut file = File::create(path).unwrap();
    let buf = generate(song);
    if song.debug {
        dump_midi(&buf, true);
    }
    file.write(buf.as_ref()).unwrap();
    file.flush().unwrap();
}
