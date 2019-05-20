extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use failure::Error;


pub fn convert(input_path: &PathBuf, output_path: &PathBuf, seconds: f64) -> Result<i32, Error> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);

    let mut out = File::create(output_path)?;

    let timing = Regex::new(r"(\d{2}:\d{2}:\d{2}[,.]\d{3}) --> (\d{2}:\d{2}:\d{2}[,.]\d{3})$")?;

    let mut skip: bool = false;
    let mut deleted_subs = 0;

    for line in reader.lines() {
        let old_line = line?;
        let is_timeline: bool = timing.is_match(&old_line);
        let mut new_line;

        if is_timeline {
            let time_line = old_line.replace(",", ".");
            // Return the capture groups corresponding to the leftmost first match:
            let caps = timing.captures(&time_line).unwrap();
            // Extract start and stop times, notice 2 ways of doing this:
            let start_string = caps.get(1).unwrap().as_str();
            let end_string = caps.get(2).map_or("", |m| m.as_str());
            new_line = build_line(start_string, end_string, seconds);

            if new_line == "(DELETED)\n" {
                deleted_subs += 1;
                skip = true;
            } else if output_path.extension().unwrap() == "srt" {
                // Convert back to '.srt' style:
                new_line = new_line.replace(".", ",");
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
                new_line = old_line;
            }
        };

        // Add \n to the lines before writing them:
        out.write((new_line + "\n").as_bytes())?;
    }

    return Ok(deleted_subs);
}

fn build_line(start_string: &str, end_string: &str, seconds: f64) -> String {
    let start_secs = get_secs(start_string);
    let end_secs = get_secs(end_string);

    let start_string = build_time_string(start_secs, seconds);
    let end_string = build_time_string(end_secs, seconds);

    let line = if start_string == "(DELETED)\n" {
        if end_string == "(DELETED)\n" {
            end_string
        } else {
            format!("00:00:00.000 --> {}", end_string)
        }
    } else {
        format!("{} --> {}", start_string, end_string)
    };

    return line;
}

fn get_secs(time_string: &str) -> f64 {
    time_string.rsplit(":")
        .map(|t| t.parse::<f64>().unwrap())
        .zip(&[1.0, 60.0, 3600.0])
        .map(|(a, b)| a * b)
        .sum()
}

fn build_time_string(seconds: f64, incr: f64) -> String {
    // incr can be negative, so the new time could be too:
    let new_time = seconds + incr;

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
