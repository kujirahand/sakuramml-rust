use std::fs;
use std::path::Path;
// use std::process::Command;

fn main() {
    let build_file = "build_number.txt";
    let build_number = if Path::new(build_file).exists() {
        fs::read_to_string(build_file)
            .unwrap_or_else(|_| "0".to_string())
            .trim()
            .parse::<u32>()
            .unwrap_or(0) + 1
    } else {
        1
    };

    fs::write(build_file, build_number.to_string()).expect("Failed to write build number");

    println!("cargo:rustc-env=BUILD_NUMBER={}", build_number);
}
