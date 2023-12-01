use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Trebuchet?!";
const PROBLEM_INPUT_FILE: &str = "./input/day01.txt";
const PROBLEM_DAY: u64 = 1;

lazy_static! {
    static ref REGEX_DIGITS: Regex = Regex::new(r"([0-9])").unwrap();
}

/// Processes the AOC 2023 Day 01 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2023 Day {PROBLEM_DAY} - \"{PROBLEM_NAME}\"");
    println!("[+] Part 1: {p1_solution}");
    println!("[+] Part 2: {p2_solution}");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {input_parser_duration:.2?}");
    println!("[+] Part 1: {p1_duration:.2?}");
    println!("[+] Part 2: {p2_duration:.2?}");
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2023 Day 01 input file in the format required by the solver functions.
///
/// Returned value is vector of strings given by the lines of the input file.
fn process_input_file(filename: &str) -> Vec<String> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>()
}

/// Solves AOC 2023 Day 01 Part 1.
///
/// Determines the sum of the calibration values extracted from the input strings. The calibration
/// values are found by extracting and combining the two digits located in each respective input
/// string.
fn solve_part1(input: &[String]) -> u64 {
    input
        .iter()
        .map(|s| extract_calibration_value(s).unwrap())
        .sum()
}

/// Solves AOC 2023 Day 01 Part 2.
///
/// ###
fn solve_part2(_input: &[String]) -> String {
    String::from("")
}

/// Extracts the calibration value from the given string.
///
/// Returns None if the string is in the incorrect format and does not contain a calibration value.
fn extract_calibration_value(s: &str) -> Option<u64> {
    let mut caps = REGEX_DIGITS.find_iter(s);
    let first_match = caps.next();
    let last_match = caps.last();
    // Check if there are no matches
    first_match.as_ref()?;
    // Extract first and last digits
    let first_digit = first_match.unwrap().unwrap().as_str().to_string();
    // Last digit is same as the first digit if there is only one digit in the input string
    let last_digit = {
        if let Some(cap) = last_match {
            cap.unwrap().as_str().to_string()
        } else {
            first_digit.to_string()
        }
    };
    let value = format!("{}{}", first_digit, last_digit)
        .parse::<u64>()
        .unwrap();
    Some(value)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 01 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day01_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(56506, solution);
    }

    /// Tests the Day 01 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day01_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
