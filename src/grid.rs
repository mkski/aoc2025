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

const CARDINAL_NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition(pub usize, pub usize);

impl Display for GridPosition {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

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

#[derive(Debug, Clone, Copy)]
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
    pub fn parse(input: String) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for (r, row) in input.lines().enumerate() {
            grid.push(Vec::new());
            for (_, col) in row.chars().enumerate() {
                grid[r].push(col);
            }
        }
        let dimensions = Dimensions {
            rows: grid.len(),
            cols: grid[0].len(),
        };
        Grid { grid, dimensions }
    }

    pub fn find_cells<P>(&self, predicate: P) -> Vec<GridCell>
    where
        P: Fn(&GridCell) -> bool,
    {
        self.iter_cells().filter(|cell| predicate(cell)).collect()
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

    pub fn iter_neighbors(&self, position: GridPosition) -> impl Iterator<Item = GridCell> {
        NEIGHBORS.into_iter().filter_map(move |(nr, nc)| {
            let neighbor_position = GridPosition(
                position.0.wrapping_add(nr as usize),
                position.1.wrapping_add(nc as usize),
            );
            self.get_cell(neighbor_position)
        })
    }

    pub fn iter_neighbors_with<P>(
        &self,
        neighbors: Vec<(i32, i32)>,
        position: GridPosition,
        predicate: &P,
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
            if let Some(cell) = self.get_cell(neighbor_position)
                && predicate(&current_cell, &cell, self)
            {
                Some(cell)
            } else {
                None
            }
        })
    }

    pub fn iter_cardinal_neighbors_with<P>(
        &self,
        position: GridPosition,
        predicate: &P,
    ) -> impl Iterator<Item = GridCell>
    where
        P: Fn(&GridCell, &GridCell, &Self) -> bool,
    {
        self.iter_neighbors_with(CARDINAL_NEIGHBORS.into(), position, predicate)
    }

    pub fn iter_all_neighbors_with<P>(
        &self,
        position: GridPosition,
        predicate: &P,
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

    pub fn find_paths<NP, GP>(
        &self,
        start: GridPosition,
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

        let start_cell = self.get_cell(start).unwrap();
        let path = Path {
            cell: start_cell,
            parent: None,
        };

        visited.insert(start);
        queue.push_back(path);

        while let Some(path) = queue.pop_front() {
            if goal_predicate(&path.cell, &self) {
                paths.push(path);
                continue;
            }

            self.iter_cardinal_neighbors_with(path.cell.position, &neighbor_predicate)
                .for_each(|cell| {
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
