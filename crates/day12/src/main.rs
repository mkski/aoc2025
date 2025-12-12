use aoc2025::grid::Grid;
use aoc2025::utils;
use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    #[allow(unused)]
    part1: usize,
    #[allow(unused)]
    part2: usize,
}

type Shape = HashSet<(usize, usize)>;

#[derive(Debug)]
struct Space {
    grid: Grid,
    required_presents: Vec<usize>,
}

fn can_fit(space: &Space, shapes: Vec<Shape>) -> bool {
    let space_area = space.grid.cols() * space.grid.rows();
    let presents_area = shapes
        .iter()
        .enumerate()
        .map(|(idx, s)| space.required_presents[idx] * s.len())
        .sum::<usize>();

    presents_area <= space_area && shapes.len() * 9 <= space_area
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);

    let mut split: Vec<&str> = input.split("\n\n").collect();
    let grids = split.pop().unwrap();
    let shapes = split
        .iter()
        .map(|&s| {
            let mut lines = s.lines();
            let _ = lines.next();
            let mut cells: Shape = HashSet::new();
            for (r, row) in lines.enumerate() {
                for (c, col) in row.chars().enumerate() {
                    if col == '#' {
                        cells.insert((r, c));
                    }
                }
            }
            cells
        })
        .collect::<Vec<Shape>>();

    let spaces = grids.lines().map(|line| {
        let (dim, rest) = line.split_once(':').unwrap();
        let (cols, rows) = dim.split_once('x').unwrap();
        let required_presents = rest
            .trim()
            .split(' ')
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        let grid: Grid = (
            rows.parse::<usize>().unwrap(),
            cols.parse::<usize>().unwrap(),
        )
            .into();
        Space {
            grid,
            required_presents,
        }
    });

    let solution = Solution {
        part1: spaces.filter(|p| can_fit(p, shapes.clone())).count(),
        part2: 0,
    };
    println!("{solution:?}");
}
