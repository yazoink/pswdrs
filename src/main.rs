use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    env,
};
use rand::seq::SliceRandom;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let file_name = "words.txt";
    let mut len = 3;
    if argv.len() > 1 {
        if argv.len() == 3 {
            match argv[1].as_str() {
                "-n" | "--number" => {
                    len = match argv[2].parse() {
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Failed to parse number: {}", e);
                            std::process::exit(1);
                        }
                    };
                },
                _ => {
                    eprintln!("Argument(s) invalid.");
                    std::process::exit(1);
                }
            }
        } else {
            match argv[1].as_str() {
                "-h" | "--help" => help(),
                _ => {
                    eprintln!("Argument(s) invalid.");
                    std::process::exit(1);
                }
            }
        }
    }
    let words = array_from_file(file_name);
    let password = generate_password(&words, len);
    println!("{}", password);
}

fn array_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Cannot locate file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}

fn help() {
    println!(r#"pswdrs -- yazoink 2024
usage: pswdrs [option] [arg]
options:
    -n, --number: specify number of words to be generated
    -h, --help: display usage and options"#);
    std::process::exit(0);
}

fn generate_password(words: &[String], num: i32) -> String {
    let mut password = String::from("");
    for _ in 0..num {
        if let Some(word) = words.choose(&mut rand::thread_rng()) {
            password.push_str(&uppercase_first_letter(&word));
        }
    }
    password
}

fn uppercase_first_letter(s: &String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
