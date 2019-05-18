extern crate regex;
use regex::Regex;

use std::path::Path;
use std::path::PathBuf;


pub fn get_paths(input: &str, seconds: f64, convert: Option<&str>) -> Result<(PathBuf, PathBuf), String> {

    println!("convert1 = {:?}", convert);

    let to_ext = match convert {
        Some(ext) => {
            ext
        },
        None => {
            // Return input extension:
            &input[input.len()-3..]
        },
    };

    println!("convert2 = {}", to_ext);

    // Create full path for inputfile:
    let input_path = Path::new(input);
    // Find input filename without path:
    let input_file = match input_path.file_name() {
        Some(n) => {
            n.to_str().expect("Invalid value for '\u{001b}[33m<INPUT>\u{001b}[0m': invalid unicode")
        },
        None => {
            return Err("Invalid value for '\u{001b}[33m<INPUT>\u{001b}[0m': no file".to_owned());
        },
    };

    // Find parent: path without filename
    // => parent will be an empty string if the path consists of the filename alone
    let parent = match input_path.parent() {
        Some(n) => {
            n.to_str().expect("Invalid value for '\u{001b}[33m<INPUT>\u{001b}[0m': invalid unicode")
        },
        None => {
            return Err("Invalid value for '\u{001b}[33m<INPUT>\u{001b}[0m': incorrect path".to_owned());
        },
    };

    // Create name for ouputfile:
    let outputfile = name_output(input_file, seconds, to_ext);

    // Create full path for outputfile:
    let output_path = if parent != "" {
        format!("{}/{}", parent, outputfile)
    } else {
        outputfile.to_string()
    };
    let output_path = Path::new(&output_path);
    // println!("Path: {}", output_path.display());

    return Ok( (input_path.to_owned(), output_path.to_owned()) );
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
    if from_ext != to_ext {
        let len = output.len();
        output.truncate(len - 3);
        output = output + to_ext;
    }

    return output;
}

pub fn report_error(error: &str) {
    eprintln!("\u{001b}[38;5;208mError:\u{001b}[0m {}\n", error);
    println!("USAGE:\n    \
                submod [FLAGS] [OPTIONS] <INPUT> <SECONDS>\n        \
                    INPUT: (Path to) .srt or .vtt subtitle file to convert\n        \
                    SECONDS: seconds to add or subtract from time encoding\n\n\
                    For more information try \u{001b}[32m--help\u{001b}[0m");
}

pub fn is_srt_or_vtt(input: String) -> Result<(), String> {
    if input.ends_with(".srt") || input.ends_with(".vtt") {
        return Ok(());
    }
    Err(String::from("incorrect file extension\n\n\
        Only \u{001b}[32m.srt\u{001b}[0m or \u{001b}[32m.vtt\u{001b}[0m files allowed."))
}

pub fn is_float(seconds: String) -> Result<(), String> {
    match seconds.parse::<f64>() {
        Ok(_) => {
            Ok(())
        },
        Err(_) => {
            Err("should be a number".to_string())
        },
    }
}

pub fn status(deleted_subs: i32, output_path: &PathBuf) {
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