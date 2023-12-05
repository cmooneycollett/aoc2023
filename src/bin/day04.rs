use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Scratchcards";
const PROBLEM_INPUT_FILE: &str = "./input/day04.txt";
const PROBLEM_DAY: u64 = 4;

lazy_static! {
    static ref REGEX_CARD: Regex = Regex::new(r"^Card\s+(\d+): (.*) \| (.*)$").unwrap();
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
fn process_input_file(filename: &str) -> HashMap<usize, usize> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .filter_map(parse_input_file_line)
        .collect::<HashMap<usize, usize>>()
}

/// Parses a line from the input file into the format required for collection into a HashMap.
fn parse_input_file_line(s: &str) -> Option<(usize, usize)> {
    if let Ok(Some(caps)) = REGEX_CARD.captures(s) {
        let card_num = caps[1].parse::<usize>().unwrap();
        let winning_nums = parse_number_set(&caps[2]);
        let game_nums = parse_number_set(&caps[3]);
        let num_overlaps = winning_nums.intersection(&game_nums).count();
        return Some((card_num, num_overlaps));
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
fn solve_part1(cards: &HashMap<usize, usize>) -> u64 {
    cards
        .iter()
        .map(|(_, &num_overlaps)| calculate_card_points(num_overlaps))
        .sum()
}

/// Solves AOC 2023 Day 04 Part 2.
///
/// Calculates the total number of scratchcards after checking all original and copied cards.
fn solve_part2(cards: &HashMap<usize, usize>) -> u64 {
    calculate_total_cards_processed(cards)
}

/// Calculates the number of points that the card is worth, based on how many of its game numbers
/// are winning numbers. The points total is calculated as 2^(n-1), where n is the number of
/// overlapping numbers.
fn calculate_card_points(num_overlaps: usize) -> u64 {
    if num_overlaps == 0 {
        return 0;
    }
    2u64.pow(u32::try_from(num_overlaps).unwrap() - 1)
}

/// Calculates the total number of scratchcards processed, including all original and copied cards.
fn calculate_total_cards_processed(cards: &HashMap<usize, usize>) -> u64 {
    let mut cards_processed = 0;
    let mut card_counts: Vec<u64> = iter::repeat(1).take(cards.len()).collect::<Vec<u64>>();
    for n in 0..cards.len() {
        // Count the copies of the current card
        cards_processed += card_counts[n];
        let card_id = n + 1;
        let winning_nums = *cards.get(&card_id).unwrap();
        // Generate a copy of the following cards for each copy of current card
        for delta in 1..=winning_nums {
            if n + delta >= card_counts.len() {
                break;
            }
            card_counts[n + delta] += card_counts[n];
        }
    }
    cards_processed
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
        let solution = solve_part2(&input);
        assert_eq!(7185540, solution);
    }
}
