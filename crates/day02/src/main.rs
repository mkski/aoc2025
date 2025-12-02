use aoc2025::utils;
use regex::Regex;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    part1: i64,
    part2: i64,
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let re = Regex::new(r"(?P<start>\d+)-(?P<end>\d+)").unwrap();

    let initial = Solution { part1: 0, part2: 0 };
    let solution = re.captures_iter(&input).fold(initial, |state, captures| {
        let start: &str = captures["start"].as_ref();
        let start_n = start.parse::<i64>().unwrap();
        let end: &str = captures["end"].as_ref();
        let end_n = end.parse::<i64>().unwrap();

        let mut part1_invalid = 0;
        let mut part2_invalid = 0;
        for i in start_n..=end_n {
            let s = i.to_string();
            let len = s.len();
            let mid = len / 2;
            let left = &s[0..mid];
            let right = &s[mid..len];

            let chars: Vec<char> = s.chars().collect();
            let mut seq_length = 1;

            for idx in 1..chars.len() / 2 {
                if chars[idx] != chars[idx % seq_length] {
                    seq_length += 1;
                }
            }

            let sequence = chars
                .chunks(seq_length)
                .reduce(|acc, chunk| if acc == chunk { chunk } else { &[] })
                .unwrap();

            if left == right {
                part1_invalid += i
            } else if !sequence.is_empty() && sequence.len() <= mid {
                part2_invalid += i
            }
        }

        Solution {
            part1: state.part1 + part1_invalid,
            part2: state.part2 + part1_invalid + part2_invalid,
        }
    });

    println!("{:?}", solution);
}
