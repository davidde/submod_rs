extern crate regex;
extern crate clap;
use clap::{App, Arg, AppSettings};

use std::error::Error;

mod convert;
mod helpers;


fn main() -> Result<(), Box<dyn Error>> {
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
        return Ok(());
    }
    let seconds = matches.value_of("SECONDS").unwrap();
    let seconds: f64 = match seconds.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("error: second argument not a number");
            helpers::help();
            return Ok(());
        },
    };

    let (input_path, output_path) = helpers::get_paths(input, seconds, matches.value_of("convert"))?;

    let deleted_subs = convert::convert(&input_path, &output_path, seconds);

    helpers::status(deleted_subs, &output_path);

    return Ok(());
}
