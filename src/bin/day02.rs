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
    static ref REGEX_GAME: Regex = Regex::new(r"^Game (\d+)").unwrap();
    static ref REGEX_RED: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref REGEX_BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
    static ref REGEX_GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
}

/// Represents the maximum number of red, blue and green cubes recorded across all groups for an
/// individual game.
struct GameCubeMax {
    red: u64,
    blue: u64,
    green: u64,
}

impl GameCubeMax {
    /// Creates a new [`GameCubeMax`] with the given cube numbers.
    fn new(red: u64, blue: u64, green: u64) -> Self {
        Self { red, blue, green }
    }

    /// Checks if the cube groups represent a possible game.
    ///
    /// A game is possible if the number of any given cube colour in a group does not exceed the
    /// maximum value for that colour.
    fn check_game(&self, max_red: u64, max_blue: u64, max_green: u64) -> bool {
        self.red <= max_red && self.blue <= max_blue && self.green <= max_green
    }

    /// Calculates the power of the game as a product of the maximum number of red, blue and green
    /// cubes.
    fn calculate_game_power(&self) -> u64 {
        self.red * self.blue * self.green
    }
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
fn process_input_file(filename: &str) -> HashMap<u64, GameCubeMax> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .filter_map(convert_line_to_game)
        .collect::<HashMap<u64, GameCubeMax>>()
}

/// Converts an input file line into tuple containing the game ID and vector of its cube groups.
fn convert_line_to_game(s: &str) -> Option<(u64, GameCubeMax)> {
    // Game match
    if let Ok(Some(game_match)) = REGEX_GAME.captures(s) {
        // Extract game ID
        let game_id = game_match[1].parse::<u64>().unwrap();
        // Find max number of red, blue and green cubes
        let max_red = REGEX_RED
            .captures_iter(s)
            .map(|cap| cap.unwrap()[1].parse::<u64>().unwrap())
            .max()
            .unwrap_or(0);
        let max_blue = REGEX_BLUE
            .captures_iter(s)
            .map(|cap| cap.unwrap()[1].parse::<u64>().unwrap())
            .max()
            .unwrap_or(0);
        let max_green = REGEX_GREEN
            .captures_iter(s)
            .map(|cap| cap.unwrap()[1].parse::<u64>().unwrap())
            .max()
            .unwrap_or(0);
        let cube_max = GameCubeMax::new(max_red, max_blue, max_green);
        return Some((game_id, cube_max));
    }
    None
}

/// Solves AOC 2023 Day 02 Part 1.
///
/// Determines the sum of the game IDs for the games that are possible, given a bag containing 12
/// red, 13 green and 14 blue cubes.
fn solve_part1(games: &HashMap<u64, GameCubeMax>) -> u64 {
    games
        .iter()
        .filter(|&(_, cube_max)| cube_max.check_game(P1_MAX_RED, P1_MAX_BLUE, P1_MAX_GREEN))
        .map(|(id, _)| id)
        .sum()
}

/// Solves AOC 2023 Day 02 Part 2.
///
/// Determines the sum of the power for each game.
///
/// The power of a game is calculated by finding the product of the minimum number of red, blue and
/// green cubes that would be required to make the game possible.
fn solve_part2(games: &HashMap<u64, GameCubeMax>) -> u64 {
    games
        .iter()
        .map(|(_, cube_max)| cube_max.calculate_game_power())
        .sum()
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
