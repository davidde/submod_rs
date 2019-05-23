extern crate regex;
extern crate clap;
use clap::{App, Arg, AppSettings};
#[macro_use]
extern crate failure;

mod submod;
mod helpers;


fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        // AllowLeadingHyphen allows passing negative seconds:
        .setting(AppSettings::AllowLeadingHyphen)
        .about("Modify the time encoding of movie subtitles.\n(UTF-8 encoded .srt or .vtt files)")
        .arg(Arg::with_name("filename")
            .help("(Path to) .srt or .vtt subtitle file to modify")
            .required(true)
            .index(1)
            .validator(helpers::is_srt_or_vtt))
        .arg(Arg::with_name("seconds")
            .help("Seconds by which to add or subtract the time encoding")
            .required(true)
            .index(2)
            .validator(helpers::is_float))
        .arg(Arg::with_name("start")
            .help("Indicates at what time the modification should start")
            .short("s") // By default, start is at the beginning of the file
            .long("start")
            .value_name("hh:mm:ss")
            .takes_value(true)
            .validator(helpers::is_timing))
        .arg(Arg::with_name("stop")
            .help("Indicates at what time the modification should stop")
            .short("S") // By default, stop is at the end of the file
            .long("stop")
            .value_name("hh:mm:ss")
            .takes_value(true)
            .validator(helpers::is_timing))
        .arg(Arg::with_name("output")
            .help("File name or path where the modified file should be stored\n\
                Note: The file name's extension takes precedence over\n      \
                      any --srt or --vtt flags")
            .short("o")
            .long("output")
            .value_name("filename")
            .takes_value(true)
            // The filename extension of `--output` takes precedence over --srt and --vtt,
            // so we don't allow combining them:
            .conflicts_with_all(&["srt", "vtt", "overwrite", "destroy"])
            // (Ideally, we should be able to notify the user with added error message!)
            .validator(helpers::is_srt_or_vtt))
        .arg(Arg::with_name("srt")
            .help("Convert to srt format")
            .long("srt")
            .display_order(3)
            .conflicts_with("vtt"))
        .arg(Arg::with_name("vtt")
            .help("Convert to vtt format")
            .long("vtt")
            .display_order(4))
        .arg(Arg::with_name("overwrite")
            .help("Overwrite input file, copying the original\n\
                (Only do this the first time; consecutive overwrites\n\
                will copy the already modified input file)")
            .short("O")
            .long("overwrite")
            .display_order(1))
        .arg(Arg::with_name("destroy")
            .help("Overwrite input file, destroying the original")
            .short("d")
            .long("destroy")
            .display_order(2));
    let matches = app.get_matches();

    // Calling .unwrap() on "INPUT" and "SECONDS" is safe because both are required arguments.
    // (If they weren't required we could use an 'if let' to conditionally get the value)
    let input = matches.value_of("filename").unwrap();
    let seconds: f64 = matches.value_of("seconds").unwrap().parse().unwrap();
    // The second unwrap call on parse() is also safe because we've already
    // validated SECONDS as a float during argument parsing (using helpers::is_float)

    // Convert begin/stop Option<&str>s to Option<f64>s:
    let (mut start_opt, mut stop_opt, mut partial) = (None, None, false);
    if let Some(time_string) = matches.value_of("start") {
        start_opt = Some(submod::get_secs(time_string));
        partial = true; // Indicate partial modification
    }
    if let Some(time_string) = matches.value_of("stop") {
        stop_opt = Some(submod::get_secs(time_string));
        partial = true;
    }

    let output_opt = matches.value_of("output");

    let mut convert_opt = None;
    if matches.is_present("srt") {
        convert_opt = Some("srt");
    }
    if matches.is_present("vtt") {
        convert_opt = Some("vtt");
    }

    let (mut overwrite, mut copy) = (false, false);
    if matches.is_present("overwrite") {
        overwrite = true;
        copy = true;
    }
    if matches.is_present("destroy") {
        overwrite = true;
    }

    let (input_path, output_path, copy_opt) = match helpers::get_paths(input, seconds,
        partial, copy, output_opt, convert_opt) {
            Ok(paths) => paths,
            Err(error) => {
                helpers::report_error(error);
                return;
            }
    };

    // Transform the file and return the number of deleted subtitles, if any:
    let deleted_subs = match submod::transform(input_path, &output_path, seconds,
        overwrite, &copy_opt, start_opt, stop_opt) {
            Ok(num) => num,
            Err(error) => {
                helpers::report_error(error);
                return;
            }
    };

    if overwrite {
        helpers::report_success(deleted_subs, input_path, overwrite, copy_opt);
    } else {
        helpers::report_success(deleted_subs, &output_path, overwrite, copy_opt);
    }
}
