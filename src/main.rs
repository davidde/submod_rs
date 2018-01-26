extern crate regex;
use regex::Regex;

extern crate clap; 
use clap::{App, Arg,};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() {
    let matches = App::new("submod")
        .version("0.1.0")
        .about("Modifies time encoding of movie subtitles")
        .author("David Deprost")
        .arg(Arg::with_name("INPUTFILE")
            .help("The .srt or .vtt inputfile to convert")
            .required(true)
            .index(1))
        .arg(Arg::with_name("SECONDS")
            .help("Seconds by which to add or subtract the time encoding")
            .required(true)
            .index(2))
        .arg(Arg::with_name("srt")
            .short("s")
            .long("srt")
            .help("Convert the .vtt file to .srt"))
        .arg(Arg::with_name("vtt")
            .short("v")
            .long("vtt")
            .help("Convert the .srt file to .vtt"))
        .get_matches();

    // Calling .unwrap() is safe here because "INPUTFILE" and "SECONDS" are required.
    // If they weren't required we could use an 'if let' to conditionally get the value,
    // like so:
    // if let Some(s) = matches.value_of("SECONDS") {
    //     println!("Value for SECONDS: {}", s);
    // }
    let inputfile = matches.value_of("INPUTFILE").unwrap();
    let seconds = matches.value_of("SECONDS").unwrap();
    let seconds: f64 = match seconds.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("error: second argument not a number");
            help();
            return;
        },
    };

    // These bools can be ignored if they are the same type as the inputfile;
    // they signify to convert to the opposite extension than the input:
    // e.g. if vtt == true, we convert the srt input to vtt.
    let vtt: bool = matches.is_present("vtt");
    let srt: bool = matches.is_present("srt");

    if inputfile.ends_with(".srt") {
        convert_srt(inputfile, seconds)
    }

    println!("Change file {} with {} seconds", inputfile, seconds);
    if vtt {
        println!("Converting the file to .vtt!");
    }
}

fn convert_srt(input: &str, _seconds: f64) {
    let f = File::open(input).expect("File not found.");
    let reader = BufReader::new(f);

    let re = Regex::new(r"\d{2}:\d{2}:\d{2},\d{3}")
        .expect("Error compiling regex.");

    for line in reader.lines() {
        let old_line = line.expect("Error reading line.");
        let timeline: bool = re.is_match(&old_line);

        if timeline {
            // println!("{}", old_line);
        }
    }

}

fn help() {
    println!("
USAGE:
    submod <INPUTFILE> <SECONDS>
        INPUTFILE: .srt or .vtt subtitle file
        SECONDS: seconds to add or subtract from time encoding
");
}
