extern crate regex;
use regex::Regex;

extern crate clap;
use clap::{App, Arg, AppSettings};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let matches = App::new("submod")
        // AllowLeadingHyphen allows passing negative seconds:
        .setting(AppSettings::AllowLeadingHyphen)
        .version("1.0.0")
        .about("Modify the time encoding of movie subtitles.\n(Only UTF-8 encoded .srt or .vtt files.)")
        .arg(Arg::with_name("INPUT")
            .help("(Path to) .srt or .vtt inputfile to convert")
            .required(true)
            .index(1))
        .arg(Arg::with_name("SECONDS")
            .help("Seconds by which to add or subtract the time encoding")
            .required(true)
            .index(2))
        .arg(Arg::with_name("convert")
            .help("Convert to .srt or .vtt format")
            .short("c")
            .long("convert")
            .value_name("srt, vtt")
            .takes_value(true))
        .get_matches();

    // Check and prepare all arguments for program start:

    // Calling .unwrap() on "INPUT" and "SECONDS" is safe because both are required arguments.
    // (If they weren't required we could use an 'if let' to conditionally get the value)
    let input = matches.value_of("INPUT").unwrap();
    if !input.ends_with(".srt") && !input.ends_with(".vtt") {
        eprintln!("error: specify either an .srt or .vtt file as input.");
        help();
        return;
    }
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

    println!("convert1 = {:?}", matches.value_of("convert"));

    // convert_to = "srt", "vtt" or "none"
    let convert_to = if let Some(s) = matches.value_of("convert") {
      let allowed = ["srt", "vtt"];
      if allowed.contains(&s) {
        s
      } else {
          eprintln!("error: conversion to .{} not supported", s);
          help();
          return;
      }
    } else {
      "none"
    };

    println!("convert2 = {}", convert_to);

    // Create full path for inputfile:
    let input_path = Path::new(input);
    // Find input filename without path:
    let input_file = match input_path.file_name() {
        Some(n) => {
            n.to_str().expect("error: invalid unicode in filename")
        },
        None => {
            eprintln!("error: incorrect path to inputfile");
            help();
            return;
        },
    };

    // Find parent: path without filename
    // => parent will be an empty string if the path consists of the filename alone
    let parent = match input_path.parent() {
        Some(n) => {
            n.to_str().expect("error: invalid unicode in path")
        },
        None => {
            eprintln!("error: incorrect path to inputfile");
            help();
            return;
        },
    };

    // Create name for ouputfile:
    let outputfile = name_output(input_file, seconds, convert_to);

    // Create full path for outputfile:
    let output_path = if parent != "" {
        format!("{}/{}", parent, outputfile)
    } else {
        outputfile.to_string()
    };
    let output_path = Path::new(&output_path);
    // println!("Path: {}", output_path.display());

    let deleted_subs = if outputfile.ends_with(".srt") {
        convert_srt(input_path, output_path, seconds)
    } else {
        convert_vtt(input_path, output_path, seconds)
    };

    status(deleted_subs, output_path);
}

fn help() {
    println!("
USAGE:
    submod <INPUTFILE> <SECONDS>
        INPUTFILE: .srt or .vtt subtitle file
        SECONDS: seconds to add or subtract from time encoding
");
}

fn name_output(input_file: &str, seconds: f64, to_ext: &str) -> String {
    // Regex to check if the inputfile was previously processed by submod:
    let pat = Regex::new(r"\{[+-]\d+\.\d+_Sec\}_")
        .expect("Error compiling regex.");

    let processed: bool = pat.is_match(input_file);
    let mut incr: f64;
    let mut output: String;

    if processed {
        // Regex for extracting the increment number from the inputfile:
        let num = Regex::new(r"[+-]\d+\.\d+")
            .expect("Error compiling regex.");

        let capture = num.captures(input_file)
            .expect("No number found in filename");
        incr = capture.get(0)
            .unwrap().as_str()
            .parse().expect("error converting number to float");

        incr += seconds;

        let index = pat.find(input_file).unwrap();
        output = "{{abc.xy}_Sec}_".to_string() + &input_file[index.end()..];
    } else {
        incr = seconds;
        output = "{{abc.xy}_Sec}_".to_string() + input_file;
    }

    if incr >= 0.0 {
        output = "{+".to_string() + &output[1..];
    }

    let incr = format!("{:.2}", incr);
    // we can't use format! because it requires a string literal as first arg;
    // so format!(output, incr) won't compile.
    output = output.replacen("{abc.xy}", &incr, 1);

    let from_ext = &input_file[input_file.len()-3..];
    if from_ext != to_ext && to_ext != "none" {
      let len = output.len();
      output.truncate(len - 3);
      output = output + to_ext;
    }

    return output;
}

fn convert_srt(input_path: &std::path::Path, output_path: &std::path::Path, seconds: f64) -> i32 {
    let f = File::open(input_path).expect("error: file not found");
    let reader = BufReader::new(f);

    let mut out = File::create(output_path)
        .expect("error creating outputfile");

    let re = Regex::new(r"\d{2}:\d{2}:\d{2},\d{3}")
        .expect("Error compiling regex");

    let mut skip: bool = false;
    let mut deleted_subs = 0;

    for line in reader.lines() {
        let old_line = line.expect("Error reading line");
        let timeline: bool = re.is_match(&old_line);

        let new_line = if timeline {
            let mut new_line = old_line.replace(",", ".");
            new_line = process_line(&new_line, seconds);
            if new_line == "(DELETED)\n" {
                deleted_subs += 1;
                skip = true;
                new_line
            } else if input_path.extension() != output_path.extension() { // return vtt:
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

    return deleted_subs;

}

fn convert_vtt(input_path: &std::path::Path, output_path: &std::path::Path, seconds: f64) -> i32 {
    let f = File::open(input_path).expect("error: file not found");
    let reader = BufReader::new(f);

    let mut out = File::create(output_path)
        .expect("error creating outputfile");

    let re = Regex::new(r"\d{2}:\d{2}:\d{2}\.\d{3}")
        .expect("Error compiling regex");

    let mut skip: bool = false;
    let mut deleted_subs = 0;

    for line in reader.lines() {
        let old_line = line.expect("Error reading line");
        let timeline: bool = re.is_match(&old_line);

        let new_line = if timeline {
            let new_line = process_line(&old_line, seconds);
            if new_line == "(DELETED)\n" {
                deleted_subs += 1;
                skip = true;
                new_line
            } else if input_path.extension() != output_path.extension() {
                // Convert back to '.srt' style:
                new_line.replace(".", ",")
            } else {
                new_line
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

    return deleted_subs;

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

fn status(deleted_subs: i32, output_path: &std::path::Path) {
    let mut text = String::new();
    if deleted_subs > 0 {
        if deleted_subs == 1 {
            text += "Success.\nOne subtitle was deleted at the beginning of the file.";
        } else {
            text = format!("Success.\n{} subtitles were deleted at the beginning of the file.",
                deleted_subs);
        }
    } else {
        text = "Success.".to_string();
    }

    println!("{}", text);
    println!("File: {}", output_path.display());
}