use aoc2025::utils;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    part1: i64,
    part2: i64,
}

fn neighbor_rolls(grid: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {
    let neighbors: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for (nr, nc) in neighbors.iter() {
        if let Some(neighbor_row) = grid.get((r as i32 + nr) as usize) {
            if let Some(neighbor) = neighbor_row.get((c as i32 + nc) as usize) {
                if *neighbor == '@' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn remove_rolls(grid: &mut Vec<Vec<char>>) -> i64 {
    let mut removed = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut to_remove: Vec<(usize, usize)> = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' && neighbor_rolls(grid, r, c) < 4 {
                to_remove.push((r, c));
            }
        }
    }

    for (r, c) in to_remove {
        grid[r][c] = '.';
        removed += 1;
    }

    removed
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut solution = Solution { part1: 0, part2: 0 };

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut removed = remove_rolls(&mut grid);
    solution.part1 += removed;
    while removed > 0 {
        solution.part2 += removed;
        removed = remove_rolls(&mut grid);
    }

    println!("{:?}", solution);
}
