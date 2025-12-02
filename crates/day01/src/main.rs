use aoc2025::utils;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    dial: i32,
    part1: i32,
    part2: i32,
}

/// Simulate a single line of input and update the solution state.
/// 'line' is expected to match the format L|R\d+.
/// 
/// ## Arguments
/// * `state` - Current solution state.
/// * `line` - A string slice representing the current line from input.
///
fn simulate_line(state: Solution, line: &str) -> Solution {
    let mut chars = line.chars();
    let direction = match chars.next().unwrap() {
        'L' => -1,
        'R' => 1,
        _ => panic!("Invalid direction"),
    };
    let mut distance: i32 = chars.collect::<String>().parse::<i32>().unwrap();
    let mut zeros = distance / 100;
    distance = distance % 100;

    let mut new_dial = state.dial + distance * direction;
    if new_dial < 0 {
        new_dial = 100 - new_dial.abs();
        // crossing zero only counts if we were previously above zero
        if state.dial > 0 {
            zeros += 1;
        }
    } else if new_dial >= 100 {
        new_dial = new_dial % 100;
        zeros += 1;
    } else if new_dial == 0 {
        zeros += 1;
    }

    Solution {
        dial: new_dial,
        part1: state.part1 + (state.dial == 0) as i32,
        part2: state.part2 + zeros,
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);

    let mut solution = Solution {
        dial: 50,
        part1: 0,
        part2: 0,
    };
    solution = input.lines().fold(solution, simulate_line);
    println!("{:?}", solution);
}
