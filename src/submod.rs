use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use regex::Regex;
use failure::Error;


pub fn transform(input_path: &Path, output_path: &Path, seconds: f64,
        start_opt: Option<f64>, stop_opt: Option<f64>)
    -> Result<i32, Error>
{
    let f = fs::File::open(input_path)?;
    let reader = BufReader::new(f);
    let timing = Regex::new(r"(\d{2}:\d{2}:\d{2}[,.]\d{3}) --> (\d{2}:\d{2}:\d{2}[,.]\d{3})$")?;

    let mut out = fs::File::create(output_path)?;
    let mut skip: bool = false;
    let mut deleted_subs = 0;

    for line in reader.lines() {
        let mut new_line = line?;
        let is_timeline: bool = timing.is_match(&new_line);

        if is_timeline {
            new_line = new_line.replace(",", ".");
            new_line = process_line(new_line, seconds, start_opt, stop_opt);

            if new_line == "(DELETED)\n" {
                deleted_subs += 1;
                skip = true; // skip/delete upcoming subtitles
            } else if output_path.extension().unwrap() == "srt" {
                // Convert back to '.srt' style:
                new_line = new_line.replace(".", ",");
            }
        } else if skip {
            // Subtitles can be 1 or 2 lines;
            // only reset skip if we have arrived at an empty line:
            if new_line == "" {
                skip = false;
            }
            continue;
        }
        // Add \n to the lines before writing them:
        out.write((new_line + "\n").as_bytes())?;
    }

    return Ok(deleted_subs);
}

fn process_line(time_line: String, seconds: f64,
    start_opt: Option<f64>, stop_opt: Option<f64>) -> String
{
    let (line_start, line_end): (f64, f64);
    // Create block so &time_line borrow ends before return:
    {
        let start_str = &time_line[0..12];
        let end_str = &time_line[17..29];

        line_start = get_secs(start_str);
        line_end = get_secs(end_str);
    }

    if let Some(start_transform) = start_opt {
        if line_end < start_transform {
            return time_line;
        }
    }
    if let Some(stop_transform) = stop_opt {
        if line_start > stop_transform {
            return time_line;
        }
    }

    let start_string = build_time_string(line_start + seconds);
    let end_string = build_time_string(line_end + seconds);

    if end_string == "(DELETED)\n" {
        end_string
    } else if start_string == "(DELETED)\n" {
        format!("00:00:00.000 --> {}", end_string)
    } else {
        format!("{} --> {}", start_string, end_string)
    }
}

/// Processes a &str of the form 'hh:mm:ss.sss'
/// into the total number of seconds as f64.
pub fn get_secs(time_string: &str) -> f64 {
    time_string.rsplit(":")
        .map(|t| t.parse::<f64>().unwrap()) // can't panic since time_string is validated by regex!
        .zip(&[1.0, 60.0, 3600.0])
        .map(|(a, b)| a * b)
        .sum()
}

fn build_time_string(seconds: f64) -> String {
    if seconds >= 0.0 {
        let hours = seconds as u64 / 3600;
        let mins = (seconds as u64 % 3600) / 60;
        let secs = seconds % 60.0;
        format!("{0:02}:{1:02}:{2:06.3}", hours, mins, secs)
    } else {
        // the subtitles are now scheduled before the start
        // of the movie, so we can delete them:
        String::from("(DELETED)\n")
    }
}
