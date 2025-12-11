use aoc2025::utils;
use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
struct Solution {
    #[allow(unused)]
    part1: usize,
    #[allow(unused)]
    part2: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct JBox(isize, isize, isize);

impl JBox {
    pub fn distance(&self, other: &Self) -> isize {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt() as isize
    }
}

fn solve(combinations: Vec<Vec<&JBox>>, box_count: usize) -> usize {
    let mut connected: Vec<HashSet<JBox>> = Vec::new();
    for combo in combinations {
        let (jbox, other) = (combo[0], combo[1]);
        let mut jbox_circuit = None;
        let mut other_circuit: Option<(usize, &mut HashSet<JBox>)> = None;
        for (idx, connected_circuit) in connected.iter_mut().enumerate() {
            if connected_circuit.contains(jbox) {
                jbox_circuit = Some(connected_circuit);
            } else if connected_circuit.contains(other) {
                other_circuit = Some((idx, connected_circuit));
            }
        }
        if let Some(jbox_circuit) = jbox_circuit {
            if let Some((other_idx, other_circuit)) = other_circuit {
                jbox_circuit.extend(other_circuit.iter());
                connected.remove(other_idx);
            } else {
                jbox_circuit.insert(*other);
            }
        } else {
            if let Some((_, other_circuit)) = other_circuit {
                other_circuit.insert(*jbox);
            } else {
                connected.push(HashSet::from([*jbox, *other]));
            }
        }
        if connected[0].len() == box_count {
            return (jbox.0 * other.0) as usize;
        }
    }
    let top3 = connected.iter().map(|c| c.len()).sorted().rev().take(3);
    top3.reduce(|a, b| a * b).unwrap()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let mut boxes: Vec<JBox> = Vec::new();
    for line in input.lines() {
        let n = line
            .split(",")
            .map(|d| d.parse().unwrap())
            .collect::<Vec<isize>>();
        boxes.push(JBox(n[0], n[1], n[2]));
    }

    let combinations = boxes
        .iter()
        .combinations(2)
        .sorted_by(|c, o| c[0].distance(c[1]).cmp(&o[0].distance(o[1])));
    let part1_combos = combinations.clone().take(1000).collect();
    let part2_combos = combinations.collect();

    let solution = Solution {
        part1: solve(part1_combos, input.lines().count()),
        part2: solve(part2_combos, input.lines().count()),
    };
    println!("{:?}", solution);
}
