extern crate regex;
use regex::Regex;

extern crate clap;
use clap::{App, Arg, AppSettings};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let matches = App::new("submod")
        // AllowLeadingHyphen allows passing negative seconds:
        .setting(AppSettings::AllowLeadingHyphen)
        .version("0.1.0")
        .about("Modify the time encoding of movie subtitles.")
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
    // e.g. if to_vtt == true, we convert the srt input to vtt.
    let to_vtt: bool = matches.is_present("vtt");
    let to_srt: bool = matches.is_present("srt");

    if inputfile.ends_with(".srt") {
        convert_srt(inputfile, seconds)
    }

    println!("Change file {} with {} seconds", inputfile, seconds);
    if to_vtt {
        println!("Converting the file to .vtt!");
    }
}

fn convert_srt(input: &str, seconds: f64) {
    let f = File::open(input).expect("File not found.");
    let reader = BufReader::new(f);

    let re = Regex::new(r"\d{2}:\d{2}:\d{2},\d{3}")
        .expect("Error compiling regex.");

    for line in reader.lines() {
        let old_line = line.expect("Error reading line.");
        let timeline: bool = re.is_match(&old_line);

        if timeline {
            let _new_line = old_line.replace(",", ".");
            //let new_line = process_line(&new_line, seconds);
            
        }
    }
    let test = process_line("00:10:12.512 --> 00:10:15.758", seconds);
    println!("Processed test: {}", test);

}

fn help() {
    println!("
USAGE:
    submod <INPUTFILE> <SECONDS>
        INPUTFILE: .srt or .vtt subtitle file
        SECONDS: seconds to add or subtract from time encoding
");
}

fn process_line(line: &str, seconds: f64) -> String {
    let start = &line[0..12];
    let start = process_time(start, seconds);

    let end = &line[17..29];
    let end = process_time(end, seconds);

    let line = if start == "(DELETED)\n" {
        if end == "(DELETED)\n" {
            String::from("(DELETED)\n")
        } else {
            format!("00:00:00.000 --> {}", end)
        }
    } else {
        format!("{} --> {}", start, end)
    };

    return line;
}

fn process_time(time: &str, incr: f64) -> String {
    let mut hours: f64 = time[0..2].parse()
        .expect("error: invalid hour field in timeline");
    hours *= 3600.0;

    let mut mins: f64 = time[3..5].parse()
        .expect("error: invalid minutes field in timeline");
    mins *= 60.0;

    let secs: f64 =  time[6..12].parse()
        .expect("error: invalid seconds field in timeline");

    // incr can be negative, so the new time could be too:
    let new_time = hours + mins + secs + incr;

    let time_string = if new_time >= 0.0 {
        let hours = new_time as u64 / 3600;
        let mins = (new_time as u64 % 3600) / 60;
        let secs = new_time % 60.0;
        format!("{0:02}:{1:02}:{2:06.3}", hours, mins, secs)
    } else {
        // the subtitles are now scheduled before the start
        // of the movie, so we can delete them:
        String::from("(DELETED)\n")
    };

    return time_string;
}