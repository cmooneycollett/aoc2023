use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Scratchcards";
const PROBLEM_INPUT_FILE: &str = "./input/day04.txt";
const PROBLEM_DAY: u64 = 4;

lazy_static! {
    static ref REGEX_CARD: Regex = Regex::new(r": (.*) \| (.*)$").unwrap();
}

/// Processes the AOC 2023 Day 04 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2023 Day 04 input file in the format required by the solver functions.
///
/// Returned value is HashMap mapping card number to tuple of its winning numbers set and game
/// numbers set.
fn process_input_file(filename: &str) -> Vec<(HashSet<u64>, HashSet<u64>)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .filter_map(parse_input_file_line)
        .collect::<Vec<(HashSet<u64>, HashSet<u64>)>>()
}

/// Parses a line from the input file into the format required for collection into a HashMap.
fn parse_input_file_line(s: &str) -> Option<(HashSet<u64>, HashSet<u64>)> {
    if let Ok(Some(caps)) = REGEX_CARD.captures(s) {
        let winning_nums = parse_number_set(&caps[1]);
        let game_nums = parse_number_set(&caps[2]);
        return Some((winning_nums, game_nums));
    }
    None
}

/// Parses the given string into a [`HashSet`] containing the unique whitespace-separated numbers
/// in the string.
fn parse_number_set(s: &str) -> HashSet<u64> {
    s.trim()
        .split_ascii_whitespace()
        .map(|s_num| s_num.parse::<u64>().unwrap())
        .collect::<HashSet<u64>>()
}

/// Solves AOC 2023 Day 04 Part 1.
///
/// Calculates the total number of points all cards are worth.
fn solve_part1(cards: &[(HashSet<u64>, HashSet<u64>)]) -> u64 {
    cards
        .iter()
        .map(|(winning_nums, game_nums)| calculate_card_points(winning_nums, game_nums))
        .sum()
}

/// Solves AOC 2023 Day 04 Part 2.
///
/// ###
fn solve_part2(_input: &[(HashSet<u64>, HashSet<u64>)]) -> u64 {
    0
}

/// Calculates the number of points that the card is worth, based on how many of its game numbers
/// are winning numbers. The points total is calculated as 2^(n-1), where n is the number of
/// overlapping numbers.
fn calculate_card_points(winning_nums: &HashSet<u64>, game_nums: &HashSet<u64>) -> u64 {
    let num_overlap = u32::try_from(winning_nums.intersection(game_nums).count()).unwrap();
    if num_overlap == 0 {
        return 0;
    }
    2u64.pow(num_overlap - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 04 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day04_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(21138, solution);
    }

    /// Tests the Day 04 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day04_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
