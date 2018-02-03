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
        let name = name_output(inputfile, seconds, to_vtt);
        println!("{}", name);
        convert_srt(inputfile, seconds, to_vtt);
    }

    println!("Change file {} with {} seconds", inputfile, seconds);
    if to_vtt {
        println!("Converting the file to .vtt!");
    }
}

fn name_output(input: &str, seconds: f64, change_ext: bool) -> String {
    // Regex to check if the inputfile was previously processed by submod:
    let pat = Regex::new(r"\{[+-]\d+\.\d+_Sec\}_")
        .expect("Error compiling regex.");

    let processed: bool = pat.is_match(input);
    let mut incr: f64 = 0.0;
    let mut output: String = "".to_string();

    if processed {
        // Regex for extracting the increment number from the inputfile:
        let num = Regex::new(r"[+-]\d+\.\d+")
            .expect("Error compiling regex.");

        let capture = num.captures(input)
            .expect("No number found in filename");
        incr = capture.get(0)
            .unwrap().as_str()
            .parse().expect("error converting number to float");

        incr += seconds;

        let index = pat.find(input).unwrap();
        output = "{{abc.xy}_Sec}_".to_string() + &input[index.end()..];
    } else {
        incr = seconds;
        output = "{{abc.xy}_Sec}_".to_string() + input;
    }

    if incr >= 0.0 {
        output = "{+".to_string() + &output[1..];
    }

    // we can't use format! because it requires a string literal as first arg;
    // so format!(output, incr) won't compile.
    let outputfile = output.replace("{abc.xy}", &incr.to_string());

    return outputfile;
}

fn convert_srt(input: &str, seconds: f64, to_vtt: bool) {
    let f = File::open(input).expect("File not found.");
    let reader = BufReader::new(f);

    let mut out = File::create("out.srt")
        .expect("error creating outputfile");

    let re = Regex::new(r"\d{2}:\d{2}:\d{2},\d{3}")
        .expect("Error compiling regex.");

    let mut skip: bool = false;
    let mut deleted_subs = 0;

    for line in reader.lines() {
        let old_line = line.expect("Error reading line.");
        let timeline: bool = re.is_match(&old_line);

        let new_line = if timeline {
            let mut new_line = old_line.replace(",", ".");
            new_line = process_line(&new_line, seconds);
            if new_line == "(DELETED)\n" {
                deleted_subs += 1;
                skip = true;
                new_line
            } else if to_vtt {
                new_line
            } else {
                // Convert back to '.srt' style:
                new_line.replace(".", ",")
            }
        } else {
            // When skip = True, subtitles are shifted too far back
            // into the past (before the start of the movie),
            // so they are deleted:
            if skip {
                // Subtitles can be 1 or 2 lines; we should only update
                // skip when we have arrived at an empty line:
                if old_line == "" {
                    skip = false;
                }
                continue;
            } else {
                old_line
            }
        };

        // Add \n to the lines before writing them:
        out.write((new_line + "\n").as_bytes())
            .expect("error writing to outputfile");
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
    // '&' is necessary here!?
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
    // '&' is not allowed here!?
    let mut hours: f64 = time[0..2].parse()
        .expect("error: invalid hour field in timeline");
    hours *= 3600.0;

    let mut mins: f64 = time[3..5].parse()
        .expect("error: invalid minutes field in timeline");
    mins *= 60.0;

    let secs: f64 = time[6..12].parse()
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