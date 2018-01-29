extern crate regex;
use regex::Regex;

extern crate clap;
use clap::{App, Arg, AppSettings};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Duration;

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
            let new_line = old_line.replace(",", ".");
            //let new_line = process_line(&new_line, seconds);
            
        }
    }
    let test = process_time("00:00:12.512", seconds);
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

// fn process_line(line: &str, seconds: f64) -> String {

// }

fn process_time(time: &str, incr: f64) -> String {
    let hours: u64 = match time[0..2].parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            panic!("error: invalid hour field in timeline");
        },
    };

    let mins: u64 = match time[3..5].parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            panic!("error: invalid minutes field in timeline");
        },
    };

    // We convert the secs to millisecs, because the Duration functions
    // require an integer u64 argument.
    let millis: u64 = match time[6..12].parse::<f64>() {
        Ok(n) => {
            let n = 1000.0 * n;
            let n = n as u64;
            n
        },
        Err(_) => {
            panic!("error: invalid seconds field in timeline");
        },
    };

    let hours = Duration::from_secs(hours*3600);
    let mins = Duration::from_secs(mins*60);
    let secs = Duration::from_millis(millis);

    let new_time = if incr >= 0.0 {
        let incr = (incr*1000.0) as u64;
        let incr = Duration::from_millis(incr);
        hours + mins + secs + incr
    } else {
        let incr = (incr*(-1000.0)) as u64;
        let incr = Duration::from_millis(incr);
        // will panic when negative!
        if hours + mins + secs < incr {
            Duration::from_millis(0)
        } else {
            hours + mins + secs - incr
        }
    };

    let new_time = new_time.as_secs() as f64
           + new_time.subsec_nanos() as f64 * 1e-9;

    let time_string = if new_time > 0.0 {
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