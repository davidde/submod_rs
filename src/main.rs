extern crate regex;
extern crate clap;
use clap::{App, Arg, AppSettings};

mod convert;
mod helpers;


fn main() {
    let app = App::new("submod")
        // AllowLeadingHyphen allows passing negative seconds:
        .setting(AppSettings::AllowLeadingHyphen)
        .version("1.0.0")
        .version_short("v")
        .about("Modify the time encoding of movie subtitles.\n(Only UTF-8 encoded .srt or .vtt files.)")
        .arg(Arg::with_name("INPUT")
            .help("(Path to) .srt or .vtt subtitle file to convert")
            .required(true)
            .index(1)
            .validator(helpers::is_srt_or_vtt))
        .arg(Arg::with_name("SECONDS")
            .help("Seconds by which to add or subtract the time encoding")
            .required(true)
            .index(2)
            .validator(helpers::is_float))
        .arg(Arg::with_name("convert")
            .help("Convert to other subtitle format")
            .short("c")
            .long("convert")
            .value_name("extension")
            .takes_value(true)
            .possible_values(&["srt", "vtt"]));
    let matches = app.get_matches();

    // Check and prepare all arguments for program start:

    // Calling .unwrap() on "INPUT" and "SECONDS" is safe because both are required arguments.
    // (If they weren't required we could use an 'if let' to conditionally get the value)
    let input = matches.value_of("INPUT").unwrap();
    let seconds: f64 = matches.value_of("SECONDS").unwrap().parse().unwrap();
    // The second unwrap call on parse() is also safe because we've already
    // validated SECONDS as a float during argument parsing (using is_float())

    let (input_path, output_path) = match helpers::get_paths(input, seconds, matches.value_of("convert")) {
        Ok(paths) => paths,
        Err(error) => {
            helpers::report_error(&error);
            return;
        }
    };

    let deleted_subs = convert::convert(&input_path, &output_path, seconds);

    helpers::status(deleted_subs, &output_path);
}
