use std::{
    cmp::max,
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
    ops::Sub,
    ptr::null,
};

use regex::Regex;

enum ReactorStatus {
    Safe,
    Unsafe,
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("File could not be opened");
    let mut reader = io::BufReader::new(file);
    let mut reactor_safe_count = 0;
    let mut reactor_unsafe_count = 0;

    loop {
        let mut line = String::new();

        if let 0 = reader.read_line(&mut line).expect("Line could not be read") {
            // Reached EOF
            break;
        }

        let reactor_levels = capture_reactor_levels(&line)
            .expect("Could not extract reactor levels from line")
            .expect("Error while parsing reactor levels");

        let reactor_status = determine_reactor_status(&reactor_levels, 1, 3)
            .expect("Reactor status could not be determined: List is empty");

        match reactor_status {
            ReactorStatus::Safe => reactor_safe_count += 1,
            ReactorStatus::Unsafe => reactor_unsafe_count += 1,
        }
    }

    println!(
        "Finished analyzing reactors: {} safe, {} unsafe",
        reactor_safe_count, reactor_unsafe_count
    );
}

fn capture_reactor_levels(line: &String) -> Option<Result<Vec<i32>, ParseIntError>> {
    let regex = Regex::new(r"\b(\d+)\b").expect("Regex could not be compiled");
    let matches = regex.captures_iter(line).map(|capture| capture.extract());
    let mut reactor_levels = Vec::<i32>::new();

    for (_, [reactor_level_str]) in matches {
        match reactor_level_str.parse::<i32>() {
            Ok(reactor_level_i32) => reactor_levels.push(reactor_level_i32),
            Err(error) => return Some(Err(error)),
        }
    }

    if reactor_levels.is_empty() {
        None
    } else {
        Some(Ok(reactor_levels))
    }
}

fn determine_reactor_status(
    reactor_levels: &[i32],
    min_difference: u32,
    max_difference: u32,
) -> Option<ReactorStatus> {
    if reactor_levels.is_empty() {
        return None;
    }

    let mut reactor_levels_iter = reactor_levels.iter();
    let mut previous_level = reactor_levels_iter.next().unwrap();
    let mut previous_increasing = true;

    // Always safe for one level
    for (index, current_level) in reactor_levels_iter.enumerate() {
        // Calculate difference and check if series is increasing
        let difference = current_level.abs_diff(*previous_level);
        let current_increasing = current_level - previous_level > 0;

        // Check if increment is not within range and allowed direction
        if (difference < min_difference || difference > max_difference)
            || (index > 0 && current_increasing != previous_increasing)
        {
            return Some(ReactorStatus::Unsafe);
        }

        previous_level = current_level;
        previous_increasing = current_increasing;
    }

    Some(ReactorStatus::Safe)
}
