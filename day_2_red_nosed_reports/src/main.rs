use std::{
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
};

use regex::Regex;

#[derive(Clone, PartialEq)]
enum ReactorStatus {
    Safe,
    Unsafe,
    None,
}

#[derive(Clone)]
struct ReactorData {
    levels: Vec<i32>,
    errors: u32,
    status: ReactorStatus,
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("File could not be opened");
    let mut reader = io::BufReader::new(file);
    let mut reactor_safe_count = 0;
    let mut reactor_unsafe_count = 0;

    loop {
        let mut line = String::new();

        if reader.read_line(&mut line).expect("Line could not be read") == 0 {
            // Reached EOF
            break;
        }

        let reactor_levels = capture_reactor_levels(&line)
            .expect("Could not extract reactor levels from line")
            .expect("Error while parsing reactor levels");

        let reactor_status = determine_reactor_status(&reactor_levels, &1, &3, &1);

        match reactor_status {
            ReactorStatus::Safe => reactor_safe_count += 1,
            ReactorStatus::Unsafe => {
                println!("UNSAFE: {}", line);
                reactor_unsafe_count += 1
            }
            ReactorStatus::None => {
                eprintln!("Reactor status could not be determined: List is empty")
            }
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
    reactor_levels: &Vec<i32>,
    min_difference: &u32,
    max_difference: &u32,
    error_tolerance: &u32,
) -> ReactorStatus {
    if reactor_levels.is_empty() {
        return ReactorStatus::None;
    }

    let reactor_data_clone = ReactorData {
        levels: reactor_levels.clone(),
        errors: 0,                   // No errors yet, need to check recursively
        status: ReactorStatus::None, // Cannot determine status yet
    };

    // Enter recursion
    let reactor_data_check = check_reactor_levels(
        &reactor_data_clone,
        min_difference,
        max_difference,
        error_tolerance,
    );

    println!("");
    reactor_data_check.status
}

// Only safe to call if reactor_data does not contain empty list!
fn check_reactor_levels(
    reactor_data: &ReactorData,
    min_difference: &u32,
    max_difference: &u32,
    error_tolerance: &u32,
) -> ReactorData {
    print!("reactor_data: ");
    for level in &reactor_data.levels {
        print!("{} ", level)
    }
    println!("");

    let mut reactor_data_clone = reactor_data.clone();
    let mut reactor_levels_iter = reactor_data.levels.iter();
    let mut prev_level = reactor_levels_iter.next().unwrap();
    let mut prev_increasing = None;

    for (index, level) in reactor_levels_iter.enumerate() {
        // Calculate difference and check if series is increasing
        let difference = level.abs_diff(*prev_level);
        let increasing = level - prev_level > 0;

        // Check if increment is not within range and allowed direction
        if (difference < *min_difference || difference > *max_difference)
            || (prev_increasing != None && increasing != prev_increasing.unwrap())
        {
            // Unsafe level: Enter recursion again
            if reactor_data.errors < *error_tolerance {
                // Check all 3 cases (current/previous/two previous)
                for offset in 0..3 {
                    if offset > 1 && index < 1 {
                        break;
                    }

                    reactor_data_clone = reactor_data.clone();
                    reactor_data_clone.levels.remove(index + 1 - offset);
                    reactor_data_clone.errors += 1; // Increase error count

                    print!("Case {} ", offset + 1);
                    let reactor_data_check = check_reactor_levels(
                        &reactor_data_clone,
                        &min_difference,
                        &max_difference,
                        &error_tolerance,
                    );

                    if reactor_data_check.status == ReactorStatus::Safe {
                        return reactor_data_check;
                    }
                }
            }

            // Reached error tolerance, reactor is unsafe
            reactor_data_clone.status = ReactorStatus::Unsafe;
            return reactor_data_clone;
        }

        // Set variables for check in next iteration
        prev_level = level;
        prev_increasing = Some(increasing);
    }

    // All levels safe
    reactor_data_clone.status = ReactorStatus::Safe;
    return reactor_data_clone;
}
