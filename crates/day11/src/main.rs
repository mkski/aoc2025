use aoc2025::utils;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    #[allow(unused)]
    part1: usize,
    #[allow(unused)]
    part2: usize,
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn count_paths<'a>(
    nodes: &Graph<'a>,
    start: &'a str,
    goal: &'a str,
    memo: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    if let Some(cached) = memo.get(&(start, goal)) {
        return *cached;
    }

    let mut paths = 0;
    if start == goal {
        paths += 1;
    } else if let Some(neighbors) = nodes.get(start) {
        for &neighbor in neighbors {
            paths += count_paths(nodes, neighbor, goal, memo);
        }
    }

    memo.insert((start, goal), paths);
    paths
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);

    let mut nodes: Graph = HashMap::new();

    for line in input.lines() {
        let (node, neighbors) = line.split_once(":").unwrap();
        nodes.insert(node, neighbors.trim().split(' ').collect());
    }

    let mut memo: HashMap<(&str, &str), usize> = HashMap::new();
    let solution = Solution {
        part1: count_paths(&nodes, "you", "out", &mut memo),
        part2: vec![
            count_paths(&nodes, "svr", "fft", &mut memo),
            count_paths(&nodes, "fft", "dac", &mut memo),
            count_paths(&nodes, "dac", "out", &mut memo),
        ].iter().fold(1, |acc, b| acc * b),
    };
    println!("{solution:?}");
}
