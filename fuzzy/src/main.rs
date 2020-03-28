use std::io::{self, BufRead};
use std::process;

mod file;
mod full_string;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == String::from("-v") {
            version();
            process::exit(0);
        } else if args[1] == String::from("-h") {
            help();
            process::exit(0);
        }
    }
    let match_type = args.get(1).unwrap();
    let query = args.get(2).unwrap();

    let stdin = io::stdin();
    let mut input: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        input.push(format!("{}", line.unwrap()));
    }


    if *match_type == "file".to_string() {
        println!("{}", file::fuzzy_match(input, &query).join("\n"));
    }else if *match_type == "full_string".to_string() {
        println!("{}", full_string::fuzzy_match(input, &query).join("\n"));
    }
}

fn version() {
    let version = "v0.1.0";
    println!("fuzzy: {}", version);
}

fn help() {
    version();
    println!("{}", "USAGE");
    println!("{}", "      fuzzy <type> <context>");
}
