use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Cube Conundrum";
const PROBLEM_INPUT_FILE: &str = "./input/day02.txt";
const PROBLEM_DAY: u64 = 2;

/// Maximum number of red cubes allowed across a game in Part 1.
const P1_MAX_RED: u64 = 12;
/// Maximum number of green cubes allowed across a game in Part 1.
const P1_MAX_GREEN: u64 = 13;
/// Maximum number of blue cubes allowed across a game in Part 1.
const P1_MAX_BLUE: u64 = 14;

lazy_static! {
    static ref REGEX_GAME: Regex = Regex::new(r"^Game (\d+): (.*)$").unwrap();
    static ref REGEX_RED: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref REGEX_BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
    static ref REGEX_GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
}

/// Represents the number of each colour of cube contained within a group.
struct CubeGroup {
    red: u64,
    blue: u64,
    green: u64,
}

/// Processes the AOC 2023 Day 02 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2023 Day 02 input file in the format required by the solver functions.
///
/// Returned value is HashMap mapping each game ID to its vector of cube groups.
fn process_input_file(filename: &str) -> HashMap<u64, Vec<CubeGroup>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .filter_map(convert_line_to_game)
        .collect::<HashMap<u64, Vec<CubeGroup>>>()
}

/// Converts an input file line into tuple containing the game ID and vector of its cube groups.
fn convert_line_to_game(s: &str) -> Option<(u64, Vec<CubeGroup>)> {
    // Game match
    if let Ok(Some(game_match)) = REGEX_GAME.captures(s) {
        // Extract game ID and split groups into strings
        let game_id = game_match[1].parse::<u64>().unwrap();
        let group_strings = game_match[2].split("; ").collect::<Vec<&str>>();
        // Extract each cube group
        let mut cube_groups: Vec<CubeGroup> = vec![];
        for s in group_strings {
            let group = convert_group_string_to_cube_group(s);
            cube_groups.push(group);
        }
        return Some((game_id, cube_groups));
    }
    None
}

/// Extracts the cube group from the given game string.
fn convert_group_string_to_cube_group(s: &str) -> CubeGroup {
    // Red
    let red = {
        if let Ok(Some(cap)) = REGEX_RED.captures(s) {
            cap[1].parse::<u64>().unwrap()
        } else {
            0
        }
    };
    // Blue
    let blue = {
        if let Ok(Some(cap)) = REGEX_BLUE.captures(s) {
            cap[1].parse::<u64>().unwrap()
        } else {
            0
        }
    };
    // Green
    let green = {
        if let Ok(Some(cap)) = REGEX_GREEN.captures(s) {
            cap[1].parse::<u64>().unwrap()
        } else {
            0
        }
    };
    // Create new cube group
    CubeGroup { red, blue, green }
}

/// Solves AOC 2023 Day 02 Part 1.
///
/// Determines the sum of the game IDs for the games that are possible, given a bag containing 12
/// red, 13 green and 14 blue cubes.
fn solve_part1(games: &HashMap<u64, Vec<CubeGroup>>) -> u64 {
    games
        .iter()
        .filter(|&(_id, groups)| check_game(groups, P1_MAX_RED, P1_MAX_GREEN, P1_MAX_BLUE))
        .map(|(id, _groups)| id)
        .sum()
}

/// Solves AOC 2023 Day 02 Part 2.
///
/// Determines the sum of the power for each game.
///
/// The power of a game is calculated by finding the product of the minimum number of red, blue and
/// green cubes that would be required to make the game possible.
fn solve_part2(games: &HashMap<u64, Vec<CubeGroup>>) -> u64 {
    games
        .iter()
        .map(|(_id, groups)| calculate_game_power(groups))
        .sum()
}

/// Checks if the cube groups represent a possible game.
///
/// A game is possible if the number of any given cube colour in a group does not exceed the maximum
/// value for that colour.
fn check_game(cube_groups: &[CubeGroup], max_red: u64, max_green: u64, max_blue: u64) -> bool {
    for group in cube_groups {
        if group.red > max_red || group.green > max_green || group.blue > max_blue {
            return false;
        }
    }
    true
}

/// Calculates the power of the game with the given cube groups.
///
/// The power is calculated by finding the product of the minimum number of red, blue and green
/// cubes that would be required to make the game possible.
fn calculate_game_power(cube_groups: &[CubeGroup]) -> u64 {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;
    for group in cube_groups {
        // Check red for new max
        if group.red > max_red {
            max_red = group.red;
        }
        // Check blue for new max
        if group.blue > max_blue {
            max_blue = group.blue;
        }
        // Check green for new max
        if group.green > max_green {
            max_green = group.green;
        }
    }
    max_red * max_blue * max_green
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 02 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day02_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(2239, solution);
    }

    /// Tests the Day 02 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day02_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(83435, solution);
    }
}
