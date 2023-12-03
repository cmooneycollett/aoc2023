use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Gear Ratios";
const PROBLEM_INPUT_FILE: &str = "./input/day03.txt";
const PROBLEM_DAY: u64 = 3;

lazy_static! {
    /// Matches any string containing one or more digits in sequence
    static ref REGEX_NUMBER: Regex = Regex::new(r"(\d+)").unwrap();
    /// Matches any single character that is not '.' or a digit
    static ref REGEX_SYMBOL: Regex = Regex::new(r"([^\.\d])").unwrap();
}

#[derive(Copy, Clone)]
struct Number {
    value: u64,
    start: usize,
    end: usize,
    counted: bool,
}

/// Processes the AOC 2023 Day 03 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2023 Day 03 input file in the format required by the solver functions.
///
/// Returned value is HashMap mapping locations to the component held at the location in the engine
/// schematic.
fn process_input_file(filename: &str) -> (Vec<Vec<Number>>, HashMap<Point2D, char>) {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut number_data: Vec<Vec<Number>> = vec![];
    let mut symbol_locs: HashMap<Point2D, char> = HashMap::new();
    for (y, row) in raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        let mut current_row: Vec<Number> = vec![];
        // Find numbers in current row
        for number_match in REGEX_NUMBER.find_iter(row) {
            let number_match = number_match.unwrap();
            let value = number_match.as_str().parse::<u64>().unwrap();
            let number_comp = Number {
                value,
                start: number_match.start(),
                end: number_match.end() - 1,
                counted: false,
            };
            current_row.push(number_comp);
        }
        number_data.push(current_row);
        // Find symbols in current row
        for symbol_match in REGEX_SYMBOL.find_iter(row) {
            let symbol_match = symbol_match.unwrap();
            let loc = Point2D::new(
                i64::try_from(symbol_match.start()).unwrap(),
                i64::try_from(y).unwrap(),
            );
            let c = symbol_match.as_str().chars().next().unwrap();
            symbol_locs.insert(loc, c);
        }
    }
    (number_data, symbol_locs)
}

/// Solves AOC 2023 Day 03 Part 1.
///
/// Add up the total of all part numbers from the engine schematic.
fn solve_part1(input: &(Vec<Vec<Number>>, HashMap<Point2D, char>)) -> u64 {
    let mut numbers = input.0.clone();
    let symbol_locs = &input.1;
    // Calculate part number sum
    let mut part_sum = 0;
    for loc in symbol_locs.keys() {
        for s_loc in loc.get_surrounding_points() {
            // Look for number in row of the surrounding point
            let check_row = usize::try_from(s_loc.y()).unwrap();
            let check_col = usize::try_from(s_loc.x()).unwrap();
            for number in numbers[check_row].iter_mut() {
                // Check if the number has already been counted
                if number.counted {
                    continue;
                }
                // Check if the number is adjacent to the symbol
                if number.start <= check_col && number.end >= check_col {
                    part_sum += number.value;
                    number.counted = true;
                }
            }
        }
    }
    part_sum
}

/// Solves AOC 2023 Day 03 Part 2.
///
/// Finds the sum of all gear ratios in the engine schematic. Gear ratios are found by calculating
/// the product of the two values adjacent to a '*' symbol, where only two values are adjacent to
/// the symbol.
fn solve_part2(input: &(Vec<Vec<Number>>, HashMap<Point2D, char>)) -> u64 {
    let symbol_locs = &input.1;
    // Calculate gear ratio sum
    let mut gear_ratio_sum = 0;
    'outer: for (&loc, &symbol) in symbol_locs {
        // Skip any symbols that are not '*'
        if symbol != '*' {
            continue;
        }
        // Refresh the state of numbers for each '*' symbol
        let mut numbers = input.0.clone();
        let mut part_numbers: Vec<u64> = vec![];
        for s_loc in loc.get_surrounding_points() {
            // Look for number in row of the surrounding point
            let check_row = usize::try_from(s_loc.y()).unwrap();
            let check_col = usize::try_from(s_loc.x()).unwrap();
            for number in numbers[check_row].iter_mut() {
                // Do not double-count numbers for the current '*' - independent of other '*'
                if number.counted {
                    continue;
                }
                // Check if part number is next to the '*' symbol
                if number.start <= check_col && number.end >= check_col {
                    number.counted = true;
                    part_numbers.push(number.value);
                }
                // Check if too many part numbers are next to the '*' symbol
                if part_numbers.len() > 2 {
                    continue 'outer;
                }
            }
        }
        // Check if correct number of gear values is next to the symbol
        if part_numbers.len() != 2 {
            continue 'outer;
        }
        // Add the gear ratio for the current gear
        gear_ratio_sum += part_numbers.iter().product::<u64>();
    }
    gear_ratio_sum
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 03 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day03_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(544664, solution);
    }

    /// Tests the Day 03 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day03_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(84495585, solution);
    }

    /// Tests the Day 03 Part 1 solver method against the 01 test input.
    #[test]
    fn test_day03_part1_ex01() {
        let input = process_input_file("./input/test/day03_01.txt");
        let solution = solve_part1(&input);
        assert_eq!(4361, solution);
    }

    /// Tests the Day 03 Part 2 solver method against the 01 test input.
    #[test]
    fn test_day03_part2_ex01() {
        let input = process_input_file("./input/test/day03_01.txt");
        let solution = solve_part2(&input);
        assert_eq!(467835, solution);
    }
}
