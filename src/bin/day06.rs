use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Wait For It";
const PROBLEM_INPUT_FILE: &str = "./input/day06.txt";
const PROBLEM_DAY: u64 = 6;

/// Processes the AOC 2023 Day 06 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2023 Day 06 input file in the format required by the solver functions.
///
/// Returned value is tuple containing the race times and best distances for the races.
fn process_input_file(filename: &str) -> (Vec<u64>, Vec<u64>) {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut lines = raw_input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>();
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>();
    (times, distances)
}

/// Solves AOC 2023 Day 06 Part 1.
///
/// Determines the product of the number of ways the best distance for each race can be beaten.
fn solve_part1((times, distances): &(Vec<u64>, Vec<u64>)) -> usize {
    times
        .iter()
        .zip(distances.iter())
        .map(|(&t_race, &d_best)| calculate_num_ways_to_beat_best_distance(t_race, d_best))
        .product()
}

/// Calculates the number of ways to beat the best distance for a race of the specified duration (in
/// milliseconds).
fn calculate_num_ways_to_beat_best_distance(t_race: u64, d_best: u64) -> usize {
    let mut count = 0;
    // Consider each possible way of attempting the race - charging boat for different periods
    for t_charge in 0..=t_race {
        // Calculate the time that boat has to run and how far it will run before end of race
        let t_run = t_race - t_charge;
        let d_run = t_run * t_charge;
        if d_run > d_best {
            count += 1;
        }
    }
    count
}

/// Solves AOC 2023 Day 06 Part 2.
///
/// ###
fn solve_part2((_times, _distances): &(Vec<u64>, Vec<u64>)) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 06 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day06_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(74698, solution);
    }

    /// Tests the Day 06 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day06_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 06 Part 1 solver method against the 01 test input.
    #[test]
    fn test_day06_part1_ex01() {
        let input = process_input_file("./input/test/day06_01.txt");
        let solution = solve_part1(&input);
        assert_eq!(288, solution);
    }
}
