use std::convert::TryInto;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

use aoc2023::utils::camelcards::{Card, CardHand};

const PROBLEM_NAME: &str = "Camel Cards";
const PROBLEM_INPUT_FILE: &str = "./input/day07.txt";
const PROBLEM_DAY: u64 = 7;

lazy_static! {
    static ref REGEX_HAND_WITH_BET: Regex = Regex::new(r"^([23456789TJQKA]{5}) (\d+)$").unwrap();
}

/// Processes the AOC 2023 Day 07 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2023 Day 07 input file in the format required by the solver functions.
///
/// Returned value is vector of tuples containing the [`CardHand`] and bet amount listed on each
/// line of the input file.
fn process_input_file(filename: &str) -> Vec<(CardHand, usize)> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .filter_map(|line| parse_input_line(line.trim()))
        .collect::<Vec<(CardHand, usize)>>()
}

/// Parses an input line, returning a tuple containing the [`CardHand`] and bet amount listed in the
/// line.
///
/// Returns None if the given string is not correctly formatted.
fn parse_input_line(s: &str) -> Option<(CardHand, usize)> {
    if let Ok(Some(caps)) = REGEX_HAND_WITH_BET.captures(s) {
        let cards: [Card; 5] = caps[1]
            .chars()
            .map(|c| Card::from_char(c).unwrap())
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let card_hand = CardHand::new(cards);
        let bet_amount = caps[2].parse::<usize>().unwrap();
        return Some((card_hand, bet_amount));
    }
    None
}

/// Solves AOC 2023 Day 07 Part 1.
///
/// Determines the total winnings from the bets associated with the hands of cards.
fn solve_part1(input: &[(CardHand, usize)]) -> usize {
    let mut hands_with_bets = input.to_vec();
    hands_with_bets.sort_by(|a, b| a.0.cmp(&b.0));
    let mut total_winnings = 0;
    for (i, (_, bet)) in hands_with_bets.iter().enumerate() {
        let rank = i + 1;
        total_winnings += rank * bet;
    }
    total_winnings
}

/// Solves AOC 2023 Day 07 Part 2.
///
/// ###
fn solve_part2(_input: &[(CardHand, usize)]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 07 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day07_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(248105065, solution);
    }

    /// Tests the Day 07 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day07_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 07 Part 1 solver method against the 01 test input.
    #[test]
    fn test_day07_part1_ex01() {
        let input = process_input_file("./input/test/day07_01.txt");
        let solution = solve_part1(&input);
        assert_eq!(6440, solution);
    }
}
