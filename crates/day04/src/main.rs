use aoc2025::grid::{Grid, GridCell};
use aoc2025::utils;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    part1: i64,
    part2: i64,
}

fn remove_cells(grid: &mut Grid) -> i64 {
    grid.update_cells(
        grid.iter_cells().filter(|cell| {
            let neighbors = grid.count_neighbors_at(cell.row, cell.col, |v| v == '@');
            cell.value == '@' && neighbors < 4
        }).collect(),
        '.',
    ) as i64
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let mut solution = Solution { part1: 0, part2: 0 };
    let mut grid = Grid::parse(input);

    let mut removed = remove_cells(&mut grid);
    solution.part1 += removed as i64;

    while removed > 0 {
        solution.part2 += removed as i64;
        removed = remove_cells(&mut grid);
    }
    println!("{:?}", solution);
}
