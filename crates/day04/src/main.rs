use aoc2025::grid::{Grid, GridCell, GridPosition};
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
    let mut grid = Grid::parse(input);

    let mut removed = grid.update_cells_where('.', can_reach) as i64;
    solution.part1 += removed;

    while removed > 0 {
        solution.part2 += removed;
        removed = grid.update_cells_where('.', can_reach) as i64;
    }
    println!("{:?}", solution);

    let path = grid.bfs(
        GridPosition(0, 0),
        |&c, _| c.position == GridPosition(5, 7)
    );

    grid.update_cells('x', &path.unwrap().into());
    println!("{}", grid);
}
