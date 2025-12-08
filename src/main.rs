use aoc2025::grid::{Grid, GridCell};
use aoc2025::utils;
use std::env;

fn valid_neighbor(current_cell: &GridCell, neighbor: &GridCell, _: &Grid) -> bool {
    if !current_cell.value.is_digit(10) || !neighbor.value.is_digit(10) {
        return false;
    }
    let current_value: i8 = current_cell.value.to_string().parse().unwrap();
    let neighbor_value: i8 = neighbor.value.to_string().parse().unwrap();
    current_value + 1 == neighbor_value
}

fn reached_target(cell: &GridCell, _: &Grid) -> bool {
    cell.value == '9'
}

fn main() {
    // 2024 day 10, run with inputs/maze. throwing this here for now while experimenting
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let grid = Grid::parse(input);

    let starts = grid.find_cells(|c| c.value == '0');
    let score = starts.iter().fold(0, |state, cell| {
        let paths = grid.find_paths_bfs(cell.position, valid_neighbor, reached_target);
        for p in paths.clone() {
            println!("{}", p)
        }
        state + paths.len()
    });
    println!("{}", score);
}
