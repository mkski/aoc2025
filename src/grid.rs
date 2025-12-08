use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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

const CARDINAL_NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub enum SearchMode {
    DFS,
    BFS,
}

pub struct SearchConfig {
    pub starting_cells: Vec<GridCell>,
    pub mode: SearchMode,
    pub first_path: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition(pub usize, pub usize);

impl Display for GridPosition {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub rows: usize,
    pub cols: usize,
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.rows, self.cols)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridCell {
    pub value: char,
    pub position: GridPosition,
}

#[derive(Debug, Clone)]
pub struct Path {
    cell: GridCell,
    parent: Option<Box<Self>>,
}

impl Path {
    pub fn contains(&self, cell: GridCell) -> bool {
        let cells: Vec<GridCell> = self.into();
        cells.contains(&cell)
    }
}

impl Into<Vec<GridCell>> for Path {
    fn into(self) -> Vec<GridCell> {
        let mut current = self;
        let mut path: Vec<GridCell> = Vec::new();
        while let Some(parent) = current.parent {
            path.push(current.cell);
            current = *parent;
        }
        path.push(current.cell);
        path
    }
}

impl Into<Vec<GridCell>> for &Path {
    fn into(self) -> Vec<GridCell> {
        let mut current = self.clone();
        let mut path: Vec<GridCell> = Vec::new();
        while let Some(parent) = current.parent {
            path.push(current.cell);
            current = *parent;
        }
        path.push(current.cell);
        path
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cells: Vec<GridCell> = self.into();
        let formatted = cells
            .iter()
            .rev()
            .map(|c| c.position.to_string())
            .collect::<Vec<_>>()
            .join(" -> ");
        write!(f, "{}", formatted)
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,
    dimensions: Dimensions,
}

impl Grid {
    pub fn rows(&self) -> usize {
        self.dimensions.rows
    }

    pub fn cols(&self) -> usize {
        self.dimensions.cols
    }

    pub fn find_cell<P>(&self, predicate: P) -> Option<GridCell>
    where
        P: Fn(&GridCell, &Self) -> bool,
    {
        self.iter_cells()
            .filter(|cell| predicate(cell, self))
            .collect::<Vec<GridCell>>()
            .first()
            .copied()
    }

    pub fn find_cells<P>(&self, predicate: P) -> Vec<GridCell>
    where
        P: Fn(&GridCell, &Self) -> bool,
    {
        self.iter_cells()
            .filter(|cell| predicate(cell, self))
            .collect()
    }

    pub fn get_cell(&self, position: GridPosition) -> Option<GridCell> {
        match self.grid.get(position.0) {
            Some(row) => match row.get(position.1) {
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

    fn iter_neighbors(
        &self,
        neighbors: Vec<(i32, i32)>,
        position: GridPosition,
    ) -> impl Iterator<Item = GridCell> {
        neighbors.into_iter().filter_map(move |(nr, nc)| {
            let neighbor_position = GridPosition(
                position.0.wrapping_add(nr as usize),
                position.1.wrapping_add(nc as usize),
            );
            self.get_cell(neighbor_position)
        })
    }

    pub fn iter_cardinal_neighbors(
        &self,
        position: GridPosition,
    ) -> impl Iterator<Item = GridCell> {
        self.iter_neighbors(CARDINAL_NEIGHBORS.into(), position)
    }

    pub fn iter_all_neighbors(&self, position: GridPosition) -> impl Iterator<Item = GridCell> {
        self.iter_neighbors(NEIGHBORS.into(), position)
    }

    fn iter_neighbors_with<P>(
        &self,
        neighbors: Vec<(i32, i32)>,
        position: GridPosition,
        predicate: P,
    ) -> impl Iterator<Item = GridCell>
    where
        P: Fn(&GridCell, &GridCell, &Self) -> bool,
    {
        let current_cell = self.get_cell(position).unwrap();
        neighbors.into_iter().filter_map(move |(nr, nc)| {
            let neighbor_position = GridPosition(
                position.0.wrapping_add(nr as usize),
                position.1.wrapping_add(nc as usize),
            );
            if let Some(neighbor_cell) = self.get_cell(neighbor_position)
                && predicate(&current_cell, &neighbor_cell, self)
            {
                Some(neighbor_cell)
            } else {
                None
            }
        })
    }

    pub fn iter_cardinal_neighbors_with<P>(
        &self,
        position: GridPosition,
        predicate: P,
    ) -> impl Iterator<Item = GridCell>
    where
        P: Fn(&GridCell, &GridCell, &Self) -> bool,
    {
        self.iter_neighbors_with(CARDINAL_NEIGHBORS.into(), position, predicate)
    }

    pub fn iter_all_neighbors_with<P>(
        &self,
        position: GridPosition,
        predicate: P,
    ) -> impl Iterator<Item = GridCell>
    where
        P: Fn(&GridCell, &GridCell, &Self) -> bool,
    {
        self.iter_neighbors_with(NEIGHBORS.into(), position, predicate)
    }

    pub fn count_neighbors_with<P>(&self, position: GridPosition, predicate: P) -> usize
    where
        P: Fn(char) -> bool,
    {
        self.iter_all_neighbors(position)
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

    pub fn find_paths<NP, GP>(
        &self,
        config: SearchConfig,
        neighbor_predicate: NP,
        goal_predicate: GP,
    ) -> Vec<Path>
    where
        NP: Fn(&GridCell, &GridCell, &Self) -> bool,
        GP: Fn(&GridCell, &Self) -> bool,
    {
        let mut queue: VecDeque<Path> = VecDeque::new();
        let mut visited: HashSet<GridPosition> = HashSet::new();
        let mut paths: Vec<Path> = Vec::new();

        for start_cell in config.starting_cells {
            let path = Path {
                cell: start_cell,
                parent: None,
            };

            visited.insert(start_cell.position);
            queue.push_back(path);
        }

        'outer: while let Some(path) = match config.mode {
            SearchMode::BFS => queue.pop_front(),
            SearchMode::DFS => queue.pop_back(),
        } {
            for found_path in paths.iter() {
                if found_path.contains(path.cell) {
                    println!("{:?}", path.cell);
                    paths.push(Path {
                        cell: path.cell,
                        parent: Some(Box::new(found_path.clone())),
                    });
                    continue 'outer;
                }
            }

            if goal_predicate(&path.cell, &self) {
                paths.push(path);
                if config.first_path {
                    return paths;
                }
                continue;
            }

            self.iter_cardinal_neighbors_with(path.cell.position, &neighbor_predicate)
                .for_each(|cell| {
                    visited.insert(cell.position);
                    let new_path = Path {
                        cell,
                        parent: Some(Box::new(path.clone())),
                    };
                    queue.push_back(new_path);
                });
        }
        paths
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

#[derive(Debug)]
pub enum ParseError {
    InvalidGrid(String),
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, ParseError> {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let row_len: Option<i32> = None;
        for (r, row) in input.lines().enumerate() {
            grid.push(Vec::new());
            if let Some(len) = row_len
                && row.len() as i32 != len
            {
                return Err(ParseError::InvalidGrid("Grid has unequal columns".into()));
            }
            for (_, col) in row.chars().enumerate() {
                grid[r].push(col);
            }
        }
        let dimensions = Dimensions {
            rows: grid.len(),
            cols: grid[0].len(),
        };
        Ok(Grid { grid, dimensions })
    }
}
