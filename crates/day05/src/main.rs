use aoc2025::utils;
use regex::Regex;
use std::cmp::{max, min};
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    part1: usize,
    part2: usize,
}

#[derive(Debug, Clone, Copy)]
struct Range(usize, usize);

impl Range {
    pub fn contains(self, n: usize) -> bool {
        self.0 <= n && self.1 >= n
    }

    pub fn overlaps(self, other: Self) -> bool {
        self.0 >= other.1 || other.0 <= self.1
    }

    pub fn merge(self, other: Self) -> Vec<Self> {
        if self.overlaps(other) {
            vec![Self(min(self.0, other.0), max(self.1, other.1))]
        } else {
            vec![self, other]
        }
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let (ranges_input, ingredients_input) = input.split_once("\n\n").unwrap();
    let mut solution = Solution { part1: 0, part2: 0 };

    let ranges_regex = Regex::new(r"(?P<start>\d+)\-(?P<end>\d+)").unwrap();
    let ingredients_regex = Regex::new(r"\d+").unwrap();

    let mut ranges = ranges_regex
        .captures_iter(ranges_input)
        .map(|cap| Range(cap["start"].parse().unwrap(), cap["end"].parse().unwrap()))
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.0);

    let ingredients = ingredients_regex
        .find_iter(ingredients_input)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    solution.part1 = ingredients
        .iter()
        .filter(|i| {
            ranges
                .iter()
                .fold(false, |state, r| state || r.contains(**i))
        })
        .count() as usize;

    let merged = ranges.iter().fold(Vec::new(), |mut state, r| {
        if state.len() == 0 {
            state.push(*r);
        } else {
            let last = state.pop().unwrap();
            state.extend(last.merge(*r));
        }
        state
    });
    solution.part2 = merged.iter().map(|r| r.1 - r.0 + 1).sum();
    println!("{:?}", solution);
}
