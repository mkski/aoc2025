use aoc2025::{
    grid::{Grid, GridCell, GridPosition, SearchConfig, SearchMode},
    utils,
};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Copy)]
struct Solution {
    #[allow(unused)]
    part1: usize,
    #[allow(unused)]
    part2: usize,
}

fn will_split(cell: &GridCell, grid: &Grid) -> bool {
    if cell.value != '^' {
        return false;
    }
    let mut next_position = GridPosition(cell.position.0.wrapping_sub(1), cell.position.1);
    while let Some(next) = grid.get_cell(next_position) {
        let left = GridPosition(next.position.0, next.position.1.wrapping_sub(1));
        let right = GridPosition(next.position.0, next.position.1.wrapping_add(1));
        if next.value == 'S' {
            return true;
        } else if let Some(left_cell) = grid.get_cell(left)
            && left_cell.value == '^'
        {
            return true;
        } else if let Some(right_cell) = grid.get_cell(right)
            && right_cell.value == '^'
        {
            return true;
        } else if next.value == '^' {
            return false;
        }
        next_position = GridPosition(next.position.0.wrapping_sub(1), next.position.1);
    }
    false
}

fn next_splitter(cell: &GridCell, grid: &Grid) -> Option<GridCell> {
    let next_position = GridPosition(cell.position.0.wrapping_add(1), cell.position.1);
    let next_cell = grid.get_cell(next_position);
    if let Some(cell) = next_cell
        && cell.value == '^'
    {
        Some(cell)
    } else {
        None
    }
}

fn prev_neighbor(cell: &GridCell, neighbor: &GridCell, _: &Grid) -> bool {
    let in_prev_row = cell.position.0.wrapping_sub(neighbor.position.0) == 1;
    let in_next_col = neighbor.position.1.wrapping_sub(cell.position.1) == 1;
    let in_prev_col = cell.position.1.wrapping_sub(neighbor.position.1) == 1;
    if neighbor.value == '^' && in_prev_row {
        false // can't move up into a splitter
    } else if neighbor.value == '^' && cell.value == '.' && (in_prev_col || in_next_col) {
        true // can move right or left into a splitter
    } else if (neighbor.value == '.' || neighbor.value == 'S') && in_prev_row {
        true // can move up into an empty space or the start
    } else {
        false
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = utils::read_input(&filename);
    let grid: Grid = input.parse().unwrap();
    let start = grid.find_cell(|c, _| c.value == 'S').unwrap();
    let mut path_counts: HashMap<GridPosition, usize> = HashMap::from([(start.position, 1)]);

    for cell in grid.iter_cells() {
        if cell.position.0 == 0 {
            continue;
        }
        let neighbors = grid
            .iter_cardinal_neighbors_with(cell.position, prev_neighbor)
            .collect::<Vec<GridCell>>();
        let path_count = neighbors
            .iter()
            .map(|&n| path_counts.get(&n.position).unwrap_or(&0))
            .sum();
        path_counts.insert(cell.position, path_count);

        if let Some(splitter) = next_splitter(&cell, &grid) {
            path_counts.insert(splitter.position, path_count);
        }
    }
    let solution = Solution {
        part1: grid.find_cells(will_split).iter().count(),
        part2: grid
            .find_cells(|c, g| c.position.0 == g.rows() - 1)
            .iter()
            .map(|goal| path_counts.get(&goal.position).unwrap_or(&0))
            .sum(),
    };
    println!("{:?}", solution);
}
