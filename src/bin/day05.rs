use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "If You Give A Seed A Fertilizer";
const PROBLEM_INPUT_FILE: &str = "./input/day05.txt";
const PROBLEM_DAY: u64 = 5;

lazy_static! {
    /// Matches space-separated seed value capture group from problem input file
    static ref REGEX_SEEDS: Regex = Regex::new(r"(?m)^seeds: (.*)$").unwrap();
    /// Matches against destination range start, source range start and length from input file maps
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
    fn map_source_value_to_destination(&self, input_value: usize) -> usize {
        for (dest_range, source_range) in &self.range_mappings {
            // Check if the input value is mapped to a destination value
            if source_range.contains(&input_value) {
                let delta = input_value - source_range.start();
                return dest_range.start() + delta;
            }
        }
        // Input value was not covered by a range in the RangeMap, so it is returned unchanged
        input_value
    }

    /// Maps the input range to destination ranges where covered by the source ranges. For the
    /// components of the input range that are not covered by a source range, they are broken off
    /// and added to the output vector.
    fn map_source_range_to_destination_range(
        &self,
        input_range: &RangeInclusive<usize>,
    ) -> Vec<RangeInclusive<usize>> {
        let mut range_overlaps: Vec<RangeInclusive<usize>> = vec![];
        for (dest_range, source_range) in &self.range_mappings {
            // Check if source and input ranges do not overlap
            if input_range.end() < source_range.start() || input_range.start() > source_range.end()
            {
                continue;
            }
            // Calculate start and end of the overlap
            let overlap_start = input_range.start().max(source_range.start());
            let overlap_end = input_range.end().min(source_range.end());
            // Determine the mapped destination range
            let delta = overlap_start - source_range.start();
            let length = overlap_end - overlap_start;
            let dest_start = dest_range.start() + delta;
            let dest_end = dest_start + length;
            range_overlaps.push(dest_start..=dest_end);
            // Check if parts of the input range have not been mapped - on left and right of input
            if input_range.start() < source_range.start() {
                let left_start = *input_range.start();
                let left_end = *source_range.start() - 1;
                range_overlaps.push(left_start..=left_end);
            }
            if input_range.end() > source_range.end() {
                let right_start = *source_range.end() + 1;
                let right_end = *input_range.end();
                range_overlaps.push(right_start..=right_end);
            }
            break;
        }
        // Input range mapped at destination if the input range is not covered by range map
        if range_overlaps.is_empty() {
            range_overlaps.push(input_range.clone());
        }
        range_overlaps
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
fn process_input_file(filename: &str) -> (Vec<RangeInclusive<usize>>, Vec<RangeMap>) {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Extract seed values - treat as range start and length value pairs
    let seed_values = {
        if let Ok(Some(caps)) = REGEX_SEEDS.captures(&raw_input) {
            caps[1]
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        } else {
            vec![]
        }
    };
    let mut seed_ranges: Vec<RangeInclusive<usize>> = vec![];
    for i in (0..seed_values.len()).step_by(2) {
        let start = seed_values[i];
        let length = seed_values[i + 1];
        seed_ranges.push(start..=(start + length - 1));
    }
    // Extract range mappings
    let range_maps = raw_input
        .split("map:")
        .skip(1)
        .map(RangeMap::new)
        .collect::<Vec<RangeMap>>();
    (seed_ranges, range_maps)
}

/// Solves AOC 2023 Day 05 Part 1.
///
/// Determines the lowest location value corresponding to an initial seed value.
fn solve_part1(input: &(Vec<RangeInclusive<usize>>, Vec<RangeMap>)) -> usize {
    let (seed_ranges, range_maps) = input;
    // Extract the seed values from ranges used in Part 2
    let seeds = seed_ranges
        .iter()
        .flat_map(|range| [*range.start(), *range.end() - *range.start() + 1])
        .collect::<Vec<usize>>();
    let mut lowest_location: Option<usize> = None;
    // Consider each seed value individually for determining lowest location
    for seed in seeds {
        let mut value = seed;
        // Map the seed value through to its location value
        for range_map in range_maps {
            value = range_map.map_source_value_to_destination(value);
        }
        if lowest_location.is_none() || lowest_location.unwrap() > value {
            lowest_location = Some(value);
        }
    }
    lowest_location.unwrap()
}

/// Solves AOC 2023 Day 05 Part 2.
///
/// Determines the lowest location value corresponding to an initial seed value, where the input
/// seed value line is treated as specifying ranges of values.
fn solve_part2(input: &(Vec<RangeInclusive<usize>>, Vec<RangeMap>)) -> usize {
    let (seed_ranges, range_maps) = input;
    let mut lowest_location: Option<usize> = None;
    // Find the lowest location value for each seed value range, and overall lowest location value
    for seed_range in seed_ranges {
        // Map the seed value range through to range/s of location values
        let mut dest_ranges = vec![seed_range.clone()];
        for range_map in range_maps {
            // Get all of the ranges that the current ranges mapped to in the current range map
            let mut new_ranges: Vec<RangeInclusive<usize>> = vec![];
            for range in dest_ranges {
                let output = range_map.map_source_range_to_destination_range(&range);
                new_ranges.extend(output);
            }
            dest_ranges = new_ranges;
        }
        // Calculate the lowest location value for the starting seed value range
        let run_lowest_location = *dest_ranges.iter().map(|range| range.start()).min().unwrap();
        // Check if a new lowest location value has been found
        if lowest_location.is_none() || lowest_location.unwrap() > run_lowest_location {
            lowest_location = Some(run_lowest_location);
        }
    }
    lowest_location.unwrap()
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
        let solution = solve_part2(&input);
        assert_eq!(52210644, solution);
    }

    /// Tests the Day 05 Part 1 solver method against the 01 test input.
    #[test]
    fn test_day05_part1_ex01() {
        let input = process_input_file("./input/test/day05_01.txt");
        let solution = solve_part1(&input);
        assert_eq!(35, solution);
    }

    /// Tests the Day 05 Part 2 solver method against the 01 test input.
    #[test]
    fn test_day05_part2_ex01() {
        let input = process_input_file("./input/test/day05_01.txt");
        let solution = solve_part2(&input);
        assert_eq!(46, solution);
    }
}
