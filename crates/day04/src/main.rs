use aoc2025::grid::{Grid, GridCell};
use aoc2025::utils;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    part1: i64,
    part2: i64,
}

fn can_reach(cell: &GridCell, grid: &Grid) -> bool {
    let neighbors = grid.count_neighbors_with(cell.position, |v| v == '@');
    cell.value == '@' && neighbors < 4
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let mut solution = Solution { part1: 0, part2: 0 };
    let mut grid: Grid = input.parse().unwrap();

    let mut removed = grid.update_cells_where('.', can_reach) as i64;
    solution.part1 += removed;

    while removed > 0 {
        solution.part2 += removed;
        removed = grid.update_cells_where('.', can_reach) as i64;
    }
    println!("{}", grid);
    println!("{:?}", solution);
}
