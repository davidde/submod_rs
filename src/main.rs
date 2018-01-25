extern crate regex;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        help();
        return;
    }
    
    let filename = &args[1];
    let seconds = &args[2];
    let seconds: f64 = match seconds.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("Error: second argument not a number");
            help();
            return;
        },
    };
    
    if filename.ends_with(".srt") {
        convert_srt(filename, seconds)
    }

    println!("Change file {} with {} seconds", filename, seconds);

}

fn convert_srt(input: &str, _seconds: f64) {
    let f = File::open(input).expect("File not found.");
    let reader = BufReader::new(f);

    let re = Regex::new(r"\d{2}:\d{2}:\d{2},\d{3}").unwrap();

    for line in reader.lines() {
        let old_line = line.expect("Error reading line.");
        let timeline: bool = re.is_match(&old_line);
        
        if timeline {
            println!("{}", old_line);
        }
    }

}

fn help() {
    println!("
Submod

Usage: submod inputfile seconds
  inputfile: .srt or .vtt subtitle file
  seconds: seconds to add or subtract from time encoding
");
}
