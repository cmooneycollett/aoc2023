use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "If You Give A Seed A Fertilizer";
const PROBLEM_INPUT_FILE: &str = "./input/day05.txt";
const PROBLEM_DAY: u64 = 5;

lazy_static! {
    static ref REGEX_SEEDS: Regex = Regex::new(r"(?m)^seeds: (.*)$").unwrap();
    static ref REGEX_MAP_LINE: Regex = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
}

/// Combines the collection of ranges for mapping between source and destination values.
struct RangeMap {
    range_mappings: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>,
}

impl RangeMap {
    /// Creates a new [`RangeMap`] from the given newline-separated string.
    fn new(s: &str) -> Self {
        // Extract source and destination ranges from input string
        let mut range_mappings: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = vec![];
        for line in s.lines() {
            if let Ok(Some(caps)) = REGEX_MAP_LINE.captures(line) {
                // Extract values from chunk line
                let dest_start = caps[1].parse::<usize>().unwrap();
                let source_start = caps[2].parse::<usize>().unwrap();
                let range_len = caps[3].parse::<usize>().unwrap();
                // Create destination and source ranges
                let dest_range = dest_start..=(dest_start + range_len - 1);
                let source_range = source_start..=(source_start + range_len - 1);
                range_mappings.push((dest_range, source_range));
            }
        }
        Self { range_mappings }
    }

    /// Maps the given value from a source range to a destination value.
    ///
    /// Returns None if the given value does not fall within a source range for any of the range
    /// mappings.
    fn map_source_value_to_destination(&self, value: usize) -> Option<usize> {
        for (dest_range, source_range) in self.range_mappings.iter() {
            if source_range.contains(&value) {
                let delta = value - source_range.start();
                return Some(dest_range.start() + delta);
            }
        }
        None
    }
}

/// Processes the AOC 2023 Day 05 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2023 Day 05 input file in the format required by the solver functions.
///
/// Returned value is tuple containing seed values and range maps given in the input file.
fn process_input_file(filename: &str) -> (Vec<usize>, Vec<RangeMap>) {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Extract seed values
    let seeds = REGEX_SEEDS
        .captures(&raw_input)
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    // Extract range mappings
    let range_maps = raw_input
        .split("map:")
        .skip(1)
        .map(RangeMap::new)
        .collect::<Vec<RangeMap>>();
    (seeds, range_maps)
}

/// Solves AOC 2023 Day 05 Part 1.
///
/// Determines the lowest location value corresponding to an initial seed value.
fn solve_part1(input: &(Vec<usize>, Vec<RangeMap>)) -> usize {
    let (seeds, range_maps) = input;
    let mut lowest_location: Option<usize> = None;
    'outer: for &seed in seeds {
        let mut value = seed;
        for range_map in range_maps {
            if let Some(dest_value) = range_map.map_source_value_to_destination(value) {
                value = dest_value;
            } else {
                continue 'outer;
            }
        }
        if lowest_location.is_none() || lowest_location.unwrap() > value {
            lowest_location = Some(value);
        }
    }
    lowest_location.unwrap()
}

/// Solves AOC 2023 Day 05 Part 2.
///
/// ###
fn solve_part2(_input: &(Vec<usize>, Vec<RangeMap>)) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 05 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day05_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(340994526, solution);
    }

    /// Tests the Day 05 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day05_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
