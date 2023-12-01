use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Trebuchet?!";
const PROBLEM_INPUT_FILE: &str = "./input/day01.txt";
const PROBLEM_DAY: u64 = 1;

lazy_static! {
    static ref REGEX_DIGIT: Regex = Regex::new(r"([1-9])").unwrap();
    static ref REGEX_DIGIT_WORD: Regex =
        Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    static ref REGEX_DIGIT_WORD_REV: Regex =
        Regex::new(r"([1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
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
        .filter_map(|s| extract_calibration_value(s, &REGEX_DIGIT, &REGEX_DIGIT))
        .sum()
}

/// Solves AOC 2023 Day 01 Part 2.
///
/// Determines the sum of the calibration values extracted from the input strings. The calibration
/// values are found by extracting and combining the first and last digits encoded in each
/// respective input string as a digit character or number word.
fn solve_part2(input: &[String]) -> u64 {
    input
        .iter()
        .filter_map(|s| extract_calibration_value(s, &REGEX_DIGIT_WORD, &REGEX_DIGIT_WORD_REV))
        .sum()
}

/// Extracts the calibration value from the given string.
///
/// The first regex is used for looking for the first match from the beginning of the string.
/// The second regex is used for looking for the first match from the end of the string, effectively
/// the last match from the start of the string.
///
/// Returns None if the string is in the incorrect format and does not contain a calibration value.
fn extract_calibration_value(s: &str, regex_first: &Regex, regex_last: &Regex) -> Option<u64> {
    // Extract first digit
    let first_digit = {
        if let Ok(Some(first_match)) = regex_first.find(s) {
            convert_string_to_digit(first_match.as_str()).unwrap()
        } else {
            return None;
        }
    };
    // Extract second digit
    let last_digit = {
        // Reverse the input string to use the regex for checking from end
        if let Ok(Some(last_match)) = regex_last.find(&s.chars().rev().collect::<String>()) {
            // Put the matched group back into correct order for parsing to digit
            let last_match = last_match.as_str().chars().rev().collect::<String>();
            convert_string_to_digit(&last_match).unwrap()
        } else {
            return None;
        }
    };
    // Parse calibration value
    let value = format!("{}{}", first_digit, last_digit)
        .parse::<u64>()
        .unwrap();
    Some(value)
}

/// Converts the string into a character digit representation. The string can be the numeric or
/// work
fn convert_string_to_digit(s: &str) -> Option<char> {
    match s {
        "1" | "one" => Some('1'),
        "2" | "two" => Some('2'),
        "3" | "three" => Some('3'),
        "4" | "four" => Some('4'),
        "5" | "five" => Some('5'),
        "6" | "six" => Some('6'),
        "7" | "seven" => Some('7'),
        "8" | "eight" => Some('8'),
        "9" | "nine" => Some('9'),
        _ => None,
    }
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
        let solution = solve_part2(&input);
        assert_eq!(56017, solution);
    }
}
