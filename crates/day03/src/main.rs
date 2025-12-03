use aoc2025::utils;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    part1: i64,
    part2: i64,
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let initial = Solution { part1: 0, part2: 0 };
    let solution = input.lines().fold(initial, |state, line| {
        let bank: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let mut left = 0;
        let mut right = 0;
        let mut turned_on: Vec<u8> = vec![];

        for idx in 0..bank.len() - 1 {
            if bank[idx] > left {
                left = bank[idx];
                right = bank[idx + 1];
            } else if bank[idx] > right {
                right = bank[idx];
            }
        }

        let mut remaining = 12;
        let mut current_idx = 0;
        while remaining > 0 {
            let (idx, max) = bank[current_idx..bank.len() - remaining + 1]
                .iter()
                .enumerate()
                .reduce(|acc, item| if acc.1 >= item.1 { acc } else { item })
                .unwrap();
            turned_on.push(*max);
            current_idx += idx + 1;
            remaining -= 1;
        }

        if bank[bank.len() - 1] > right {
            right = bank[bank.len() - 1];
        }

        let joltage = turned_on
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        Solution {
            part1: state.part1 + 10 * left as i64 + right as i64,
            part2: state.part2 + joltage,
        }
    });
    println!("{:?}", solution);
}
