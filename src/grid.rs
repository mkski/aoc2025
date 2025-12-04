use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition(pub usize, pub usize);

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    rows: usize,
    cols: usize,
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.rows, self.cols)
    }
}

#[derive(Clone, Copy)]
pub struct GridCell {
    pub value: char,
    pub position: GridPosition,
}

#[derive(Clone)]
pub struct Path {
    cell: GridCell,
    parent: Option<Box<Self>>,
}

impl Into<Vec<GridCell>> for Path {
    fn into(self) -> Vec<GridCell> {
        let mut current = self;
        let mut path: Vec<GridCell> = Vec::new();
        while let Some(parent) = current.parent {
            path.push(current.cell);
            current = *parent;
        }
        path
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,
    dimensions: Dimensions,
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

        Grid {
            grid: grid.clone(),
            dimensions: Dimensions {
                rows: grid.len(),
                cols: grid[0].len(),
            },
        }
    }

    pub fn get_cell(&self, position: GridPosition) -> Option<GridCell> {
        match self.grid.get((position.0 as i32) as usize) {
            Some(row) => match row.get((position.1 as i32) as usize) {
                Some(value) => Some(GridCell {
                    value: *value,
                    position,
                }),
                None => None,
            },
            None => None,
        }
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = GridCell> {
        self.grid.iter().enumerate().flat_map(|(r, row)| {
            row.iter().enumerate().map(move |(c, &value)| GridCell {
                value,
                position: GridPosition(r, c),
            })
        })
    }

    pub fn iter_neighbors(&self, position: GridPosition) -> impl Iterator<Item = GridCell> {
        NEIGHBORS.into_iter().filter_map(move |(nr, nc)| {
            let neighbor_position = GridPosition(
                position.0.wrapping_add(nr as usize),
                position.1.wrapping_add(nc as usize),
            );
            self.get_cell(neighbor_position)
        })
    }

    pub fn count_neighbors_with<P>(&self, position: GridPosition, predicate: P) -> usize
    where
        P: Fn(char) -> bool,
    {
        self.iter_neighbors(position)
            .filter(|cell| predicate(cell.value))
            .count()
    }

    pub fn update_cell(&mut self, position: GridPosition, new_value: char) {
        self.grid[position.0][position.1] = new_value;
    }

    pub fn update_cells(&mut self, new_value: char, cells: &Vec<GridCell>) -> i32 {
        let mut updated = 0;
        for cell in cells {
            self.update_cell(cell.position, new_value);
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
        self.update_cells(new_value, cells_to_update)
    }

    pub fn bfs<P>(&self, start: GridPosition, predicate: P) -> Option<Path>
    where
        P: Fn(&GridCell, &Self) -> bool,
    {
        let mut queue: VecDeque<Path> = VecDeque::new();
        let mut visited: HashSet<GridPosition> = HashSet::new();

        let start_cell = self.get_cell(start).unwrap();
        let path = Path {
            cell: start_cell,
            parent: None,
        };

        visited.insert(start);
        queue.push_back(path);

        while let Some(path) = queue.pop_front() {
            if predicate(&path.cell, &self) {
                return Some(path);
            }

            self.iter_neighbors(path.cell.position).for_each(|cell| {
                if !visited.contains(&cell.position) {
                    visited.insert(cell.position);
                    let new_path = Path {
                        cell,
                        parent: Some(Box::new(path.clone())),
                    };
                    queue.push_back(new_path);
                }
            });
        }
        None
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid {}", self.dimensions)?;
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
