use std::io;
use std::io::Read;
use std::fs::File;
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rtps: Vec<&str> = args[1].split(',').collect();
    for a in rtps {
        let mut fuck = a.to_string();
        fuck.push_str("/doc/tags");
        let f = read_username_from_file(fuck.as_str());
        match f {
            Ok(f) => {
                for line in f.lines() {
                    if let Some(first_word) = line.trim().split_whitespace().next() {
                        println!("{}", first_word);
                    }
                }
            },
            _ => {}
        }
    }
}

fn read_username_from_file(fname: &str) -> Result<String, io::Error> {
    let mut f = File::open(fname)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
