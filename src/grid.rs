use std::{
    cell,
    fmt::{Display, Formatter},
};

pub struct GridCell {
    pub value: char,
    pub row: usize,
    pub col: usize,
}

pub struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn parse(input: String) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for (r, row) in input.lines().enumerate() {
            grid.push(Vec::new());
            for (_, col) in row.chars().enumerate() {
                grid[r].push(col);
            }
        }
        Grid { grid }
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = GridCell> {
        self.grid.iter().enumerate().flat_map(|(r, row)| {
            row.iter().enumerate().map(move |(c, &value)| GridCell {
                value,
                row: r,
                col: c,
            })
        })
    }

    pub fn count_neighbors_at<P>(&self, r: usize, c: usize, predicate: P) -> usize
    where
        P: Fn(char) -> bool,
    {
        let neighbors = [
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
            if let Some(neighbor_row) = self.grid.get((r as i32 + nr) as usize) {
                if let Some(neighbor) = neighbor_row.get((c as i32 + nc) as usize) {
                    if predicate(*neighbor) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn update_cell(&mut self, r: usize, c: usize, new_value: char) {
        self.grid[r][c] = new_value;
    }

    pub fn update_cells(&mut self, cells: Vec<GridCell>, new_value: char) -> i32 {
        let mut updated = 0;
        for cell in cells {
            self.update_cell(cell.row, cell.col, new_value);
            updated += 1;
        }
        updated
    }

    pub fn update_cells_where<P>(&mut self, new_value: char, predicate: P) -> i32
    where
        P: Fn(&GridCell, &Self) -> bool,
    {
        let cells_to_update = &self
            .iter_cells()
            .filter(|cell| predicate(cell, self))
            .collect::<Vec<GridCell>>();
        for cell in cells_to_update {
            self.update_cell(cell.row, cell.col, new_value);
        }
        cells_to_update.len() as i32
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
