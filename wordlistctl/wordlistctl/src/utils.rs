extern crate ansi_term;
use ansi_term::Color::*;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

macro gen_msg($text:ident, $c:literal) {
    match $c {
        "blue" => {
            println!("{}", Blue.bold().paint($text));
        },
        "red" => {
            println!("{}", Red.bold().paint($text));
        },
        "yellow" => {
            println!("{}", Yellow.bold().paint($text));
        },
        "green" => {
            println!("{}", Green.bold().paint($text));
        },
        _ => {
            println!("{}", $text);
        }
    };
}

pub(crate) fn gen_banner(string_to_use: &str) {
    gen_msg!(string_to_use, "red");
}

fn error(string_to_use: &str) {
    let msg = format!("[-] {}", string_to_use);
    gen_msg!(msg, "red");
}

fn warn(string_to_use: &str) {
    let msg = format!("[!] {}", string_to_use);
    gen_msg!(msg, "yellow");
}

fn info(string_to_use: &str) {
    let msg = format!("[*] {}", string_to_use);
    gen_msg!(msg, "blue");
}

fn success(string_to_use: &str) {
    let msg = format!("[+] {}", string_to_use);
    gen_msg!(msg, "green");
}

// load repo utility function
// private because we dont want to expose this function
// we just want load the file and return a result with the file  
fn load_repo(file: &str) -> Result<File, &'static str> {
    let json_file_path = Path::new(file);
    let file: File = File::open(json_file_path).expect("File not found");

    Ok(file)
}

fn get_file_ext(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn decompress_file(infile: &str) -> Result<(), std::io::Error> {
    let filename: String = infile.to_lowercase();
    info(format!("Decompressing {}", infile));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_error() {
        error("error");
    }

    #[test]
    fn test_warning() {
        warn("warning");
    }

    #[test]
    fn test_info() {
        info("info");
    }

    #[test]
    fn test_success() {
        success("success");
    }
}