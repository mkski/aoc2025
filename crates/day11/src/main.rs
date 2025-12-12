use aoc2025::utils;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    #[allow(unused)]
    part1: usize,
    #[allow(unused)]
    part2: usize,
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;
type Path<'a> = Vec<&'a str>;

fn dfs<'a>(nodes: Graph<'a>, start: &'a str, goal: &str) -> Vec<Path<'a>> {
    let mut paths = vec![];
    let mut queue = VecDeque::from([vec![start]]);

    while let Some(current) = queue.pop_front() {
        // println!("{current:?} {}", current.len());
        let &current_node = current.last().unwrap();
        if current_node == goal {
            paths.push(current);
            continue;
        }

        if let Some(neighbors) = nodes.get(current_node) {
            for neighbor in neighbors.iter() {
                if current.contains(neighbor) {
                    continue;
                };
                let mut new_path = current.clone();
                new_path.push(neighbor);
                queue.push_back(new_path);
            }
        }
    }
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

    // let mut memo: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    // for (&node, neighbors) in nodes.iter() {
    //     memo.insert(node, HashMap::new());
    //     for &neighbor in neighbors {
    //         memo[node].insert(neighbor, 1);
    //     }
    // }

    let solution = Solution {
        part1: dfs(nodes.clone(), "you", "out").len(),
        part2: dfs(nodes.clone(), "svr", "dac")
            .iter()
            .filter_map(|path| {
                // let mut path_nodes: HashSet<&str, _> = HashSet::new();
                // for &node in path.iter() {
                //     path_nodes.insert(node);
                // }
                // if path_nodes.contains("dac") && path_nodes.contains("fft") {
                //     Some(path)
                // } else {
                //     None
                // }
                Some(path)
            })
            .count(),
    };
    println!("{solution:?}");
}
