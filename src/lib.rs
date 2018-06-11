use std::fmt::{self, Formatter, Display};
use std::fmt::Write;

pub enum CellState {
    Alive,
    Dead,
}

pub struct Cell {
    pub state: CellState,
}

pub struct InitialBoard {
    generation: usize,
    max_life: usize,
    rows: usize,
    cols: usize,
    grid: Vec<Cell>,
}

struct Board {
    generation: usize,
    max_life: usize,
    rows: usize,
    cols: usize,
    grid: Vec<Cell>,
}

impl InitialBoard {
    pub fn new(rows: usize, cols: usize, cells: Vec<Cell>, generation: usize, max_life: usize) -> Board {
        let capacity = rows * cols;
        let mut grid = Vec::with_capacity(capacity);
        for cell in cells {
            grid.push(cell);
            if grid.len() == capacity {
                break;
            }
        };
        while grid.len() < capacity {
            grid.push(Cell { state: CellState::Dead });
        }
        Board {
            generation,
            max_life,
            rows,
            cols,
            grid,
        }
    }
    fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.grid.get(x + y * self.cols)
    }
    fn get_cell_neighbors(&self, x: usize, y: usize) -> Vec<Cell> {
        let mut cells = Vec::with_capacity(8);
        let range_x =
            if x == 0 {
                vec![x, x + 1]
            } else if x == self.cols - 1 {
                vec![x - 1, x + 1]
            } else {
                vec![x - 1, x, x + 1]
            };
        let range_y =
            if x == 0 {
                vec![x, x + 1]
            } else if x == self.rows - 1 {
                vec![x - 1, x + 1]
            } else {
                vec![x - 1, x, x + 1]
            };
        let range_cor = Vec::with_capacity(8);
        for p_x in range_x {
            for p_y in range_y {
                if p_x != x && p_y != y {
                    cells.push(self.get_cell(p_x, p_y));
                }
            }
        }
        cells
    }
}

impl Iterator for InitialBoard {
    type Item = Board;
    fn next(&mut self) -> Option<Board> {
        if self.generation >= self.max_life {
            return None;
        }
        let capacity = self.rows * self.cols;
        let mut grid = Vec::with_capacity(capacity);
        for x in 0..self.cols {
            for y in 0..self.rows {
                let cell = self.get_cell(x, y);
                let neighbors = self.get_cell_neighbors(x, y);
                let alive_neighbor_count = neighbors.iter()
                    .filter(|c| {
                        c.state == CellState::Alive
                    })
                    .collect::<Vec<Cell>>()
                    .len();
                let new_cell = match cell.state {
                    CellState::Alive => {
                        match alive_neighbor_count {
                            2 | 3 => Cell { state: CellState::Alive },
                            _ => Cell { state: CellState::Dead }
                        }
                    }
                    CellState::Dead => {
                        match alive_neighbor_count {
                            3 => Cell { state: CellState::Alive },
                            _ => Cell { state: CellState::Dead }
                        }
                    }
                };
                grid.push(new_cell);
            }
        }
        self.grid = grid;
        self.generation += 1;
        return Some(self.clone());
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Generation {}:", self.generation);
        writeln!(f, "{} {}", self.rows, self.cols);
        for x in 0..self.cols {
            for y in 0..self.rows {
                let Cell { state } = self.get_cell(x, y);
                match state {
                    CellState::Alive => f.write_char('*'),
                    CellState::Dead => f.write_char('.'),
                };
            }
            f.write_char('\n');
        }
        Ok(())
    }
}

// todo: solve the type error
// todo: refactor for better organization
// todo: add unit tests