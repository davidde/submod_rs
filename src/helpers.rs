extern crate regex;
use regex::Regex;

use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;

use failure::Error;


pub fn get_paths(input: &str, seconds: f64, convert: Option<&str>)
    -> Result<(PathBuf, PathBuf), Error>
{
    // Create full path for inputfile:
    let input_path = Path::new(input);

    // Find parent: path without filename
    // => parent will be empty if the path consists of the filename alone
    let input_parent = input_path.parent()
        .ok_or(format_err!("Invalid value for '\u{001b}[33m<INPUT>\u{001b}[0m': incorrect path"))?;

    // Find input file name without path:
    let input_file = input_path.file_name()
        // Notice difference between both `and_then` calls:
        .and_then(OsStr::to_str) // to_str returns an Option
        .and_then(|str| Some(str.to_owned())) // to_owned doesn't, so we need to manually wrap return value in Some!
        .ok_or(format_err!("Invalid value for '\u{001b}[33m<INPUT>\u{001b}[0m': invalid file name"))?;

    // Create name for output file:
    let mut output_file = input_file;
    // Change extension if necessary:
    if let Some(to_ext) = convert {
        let len = output_file.len();
        output_file.truncate(len - 3);
        output_file.push_str(to_ext);
    }
    output_file = name_output(&output_file, seconds);

    // Create full path for output file:
    let output_path = Path::new(input_parent).join(output_file);

    return Ok( ( input_path.to_owned(), output_path.to_owned() ) );
}

fn name_output(input_file: &str, seconds: f64) -> String {
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

    return output;
}

pub fn is_srt_or_vtt(input: String) -> Result<(), String> {
    if input.ends_with(".srt") || input.ends_with(".vtt") {
        return Ok(());
    }
    Err(String::from("incorrect file extension\n\n\
        Only \u{001b}[32m.srt\u{001b}[0m or \u{001b}[32m.vtt\u{001b}[0m files allowed."))
}

pub fn is_float(seconds: String) -> Result<(), String> {
    if let Ok(_) = seconds.parse::<f64>() {
        Ok(())
    } else {
        Err("should be a number".to_string())
    }
}

pub fn report_error(error: Error) {
    eprintln!("\u{001b}[38;5;208mError:\u{001b}[0m {}\n", error);
    println!("USAGE:\n    \
                submod [FLAGS] [OPTIONS] <INPUT> <SECONDS>\n        \
                    INPUT: (Path to) .srt or .vtt subtitle file to convert\n        \
                    SECONDS: seconds to add or subtract from time encoding\n\n\
                    For more information try \u{001b}[32m--help\u{001b}[0m");
}

pub fn report_success(deleted_subs: i32, output_path: &PathBuf) {
    println!("\u{001b}[32;1mSuccess.\u{001b}[0m");

    if deleted_subs > 0 {
        if deleted_subs == 1 {
            println!("    \u{001b}[41;1m ! \u{001b}[0m   One subtitle was deleted at the beginning of the file.");
        } else {
            println!("    \u{001b}[41;1m ! \u{001b}[0m   {} subtitles were deleted at the beginning of the file.",
                deleted_subs);
        }
    }

    println!(" Output: \u{001b}[1m \u{001b}[48;5;238m {} \u{001b}[0m", output_path.display());
}