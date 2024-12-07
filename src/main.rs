use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    path::PathBuf,
    env,
};
use rand::seq::SliceRandom;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let file_paths = get_file_path_array();
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
    let file_path: String = match get_file_path(file_paths) {
        Some(path) => path,
        None => {
            eprintln!("words.txt not found.");
            std::process::exit(1);
        },
    };
    let words = array_from_file(file_path);
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

fn file_exists(file_name: &str) -> bool {
    let path = Path::new(file_name);
    match path.try_exists() {
        Ok(true) => true,
        Ok(false) => false,
        Err(e) => {
            eprintln!("Error checking file existence: {}", e);
            std::process::exit(1);
        },
    }
}

fn get_file_path_array() -> Vec<String> {
    let mut file_paths: Vec<String> = Vec::new();
    if let Ok(home) = env::var("HOME") {
        let config_path = PathBuf::from(home)
            .join(".config")
            .join("pswdrs")
            .join("words.txt");
        file_paths.push(config_path.to_string_lossy().into_owned())
    }
    file_paths.push("/usr/share/pswdrs/words.txt".to_string());
    file_paths.push("data/words.txt".to_string());
    file_paths
}

fn get_file_path(file_paths: Vec<String>) -> Option<String> {
    for file_path in file_paths {
        if file_exists(&file_path) {
            return Some(file_path);
        }
    }
    None
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
