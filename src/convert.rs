extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;


pub fn convert(input_path: &PathBuf, output_path: &PathBuf, seconds: f64) -> i32 {
    let f = File::open(input_path).expect("error: file not found");
    let reader = BufReader::new(f);

    let mut out = File::create(output_path)
        .expect("error creating outputfile");

    let re = Regex::new(r"\d{2}:\d{2}:\d{2}[,.]\d{3}")
        .expect("Error compiling regex");

    let mut skip: bool = false;
    let mut deleted_subs = 0;

    for line in reader.lines() {
        let old_line = line.expect("Error reading line");
        let timeline: bool = re.is_match(&old_line);
        let mut new_line;

        if timeline {
            new_line = old_line.replace(",", ".");
            new_line = process_line(&new_line, seconds);
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
