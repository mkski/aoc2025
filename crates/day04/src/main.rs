use aoc2025::grid::Grid;
use aoc2025::utils;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    part1: i64,
    part2: i64,
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let mut solution = Solution { part1: 0, part2: 0 };
    let mut grid = Grid::parse(input);

    let mut iteration = 0;
    loop {
        let cells_to_remove = grid.iter_cells().filter(|cell| {
            let neighbors = grid.count_neighbors_at(cell.row, cell.col, |v| v == '@');
            cell.value == '@' && neighbors < 4
        });
        let removed = grid.update_cells(cells_to_remove.collect(), '.');
        if iteration == 0 {
            solution.part1 += removed as i64;
        }
        solution.part2 += removed as i64;
        if removed == 0 {
            break;
        }
        iteration += 1;
    }
    println!("{:?}", solution);
}
