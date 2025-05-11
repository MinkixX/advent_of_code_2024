use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;
mod quicksort;

enum Error {
    CountNotEqual(),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CountNotEqual() => write!(f, "the lists have to be equal lengths"),
        }
    }
}

fn main() {
    let path = "input.txt";
    let regex = Regex::new(r"^\s*(\d+)\s+(\d+)\s*$").expect("regex could not be compiled");
    let file = File::open(path);

    match file {
        Ok(handle) => {
            let mut reader = io::BufReader::new(handle);
            let mut location_ids_l = Vec::new();
            let mut location_ids_r = Vec::new();

            // 1. Read file and fill lists
            loop {
                let mut line = String::new();

                match reader.read_line(&mut line) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            // Reached EOF
                            break;
                        }

                        // Capture groups
                        match capture_location_ids(&regex, &line) {
                            Some(location_ids) => {
                                location_ids_l.push(location_ids.0);
                                location_ids_r.push(location_ids.1);
                            }
                            None => {
                                eprintln!("Could not capture locationIDs");
                            }
                        };
                    }
                    Err(error) => {
                        eprintln!("Error while reading file {}: {}", path, error);
                    }
                }
            }

            // 2. Sort the lists
            //location_ids_l.sort();
            //location_ids_r.sort();
            quicksort::sort(&mut location_ids_l);
            quicksort::sort(&mut location_ids_r);

            // 3. Calculate total distance
            match calc_location_id_distance(&location_ids_l, &location_ids_r) {
                Ok(distance) => {
                    println!("The total distance between the lists is: {}", distance);
                }
                Err(error) => {
                    println!("Error while calculating locationID distance: {}", error);
                }
            };

            // 4. Calculate similarity score
            let similarity = calc_location_id_similarity(&location_ids_l, &location_ids_r);
            println!("The total similarity between the lists is: {}", similarity);
        }
        Err(error) => {
            eprintln!("Error while opening file {}: {}", path, error);
        }
    }
}

fn capture_location_ids(regex: &Regex, line: &String) -> Option<(u32, u32)> {
    match regex.captures(&line) {
        Some(captures) => {
            let mut location_id_l = 0;
            let mut location_id_r = 0;

            // Always exactly one match with two groups -> can unwrap
            // Match left locationID
            match captures.get(1).unwrap().as_str().parse::<u32>() {
                Ok(location_id) => {
                    location_id_l = location_id;
                }
                Err(error) => {
                    eprintln!("Error while parsing left number: {}", error);
                }
            }

            // Match right locationID
            match captures.get(2).unwrap().as_str().parse::<u32>() {
                Ok(location_id) => {
                    location_id_r = location_id;
                }
                Err(error) => {
                    eprintln!("Error while parsing right number: {}", error);
                }
            }

            Some((location_id_l, location_id_r))
        }
        None => {
            eprint!("Error while extracting numbers: ");
            eprintln!("Each line must contain two numbers separated by space(s)");

            None
        }
    }
}

fn calc_location_id_distance(location_ids_l: &[u32], location_ids_r: &[u32]) -> Result<u32, Error> {
    let l_length = location_ids_l.len();
    let r_length = location_ids_r.len();

    // Check if lists have equal lengths
    if l_length != r_length {
        return Err(Error::CountNotEqual());
    }

    let mut i = 0;
    let mut distance = 0u32;

    while i < l_length && i < r_length {
        distance += location_ids_l[i].abs_diff(location_ids_r[i]);

        i += 1;
    }

    Ok(distance)
}

fn calc_location_id_similarity(location_ids_l: &[u32], location_ids_r: &[u32]) -> u32 {
    let mut location_ids_r_map = HashMap::new();
    let mut similarity = 0u32;

    for location_id_r in location_ids_r {
        let mut count = 1;

        // Check if key is already in map
        match location_ids_r_map.get(&location_id_r) {
            Some(value) => {
                // Already in map: increase count by 1
                count = value + 1;
            }
            None => {
                // Not in map: leave count at 1
            }
        }

        location_ids_r_map.insert(location_id_r, count);
    }

    for location_id_l in location_ids_l {
        match location_ids_r_map.get(&location_id_l) {
            Some(value) => {
                // Already in map: increase similarity score by locationID * count
                similarity += location_id_l * value;
            }
            None => {
                // Not in map: skip
            }
        }
    }

    similarity
}
