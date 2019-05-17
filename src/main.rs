extern crate regex;
extern crate clap;
use clap::{App, Arg, AppSettings};

use std::path::Path;

mod convert;
mod helpers;


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
        helpers::help();
        return;
    }
    let seconds = matches.value_of("SECONDS").unwrap();
    let seconds: f64 = match seconds.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("error: second argument not a number");
            helpers::help();
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
          helpers::help();
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
            helpers::help();
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
            helpers::help();
            return;
        },
    };

    // Create name for ouputfile:
    let outputfile = helpers::name_output(input_file, seconds, convert_to);

    // Create full path for outputfile:
    let output_path = if parent != "" {
        format!("{}/{}", parent, outputfile)
    } else {
        outputfile.to_string()
    };
    let output_path = Path::new(&output_path);
    // println!("Path: {}", output_path.display());

    let deleted_subs = convert::convert(input_path, output_path, seconds);

    helpers::status(deleted_subs, output_path);
}
