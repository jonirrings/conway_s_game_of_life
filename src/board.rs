use std::fmt::{self, Display, Formatter};

use ndarray::Array2;

type Grid = Array2<u8>;

pub struct Board {
    generation: usize,
    rows: usize,
    cols: usize,
    grid: Grid,
}

impl Board {
    pub fn new(rows: usize, cols: usize, cells: Vec<u8>, generation: usize) -> Board {
        let grid = Grid::from_shape_vec((rows, cols), cells).unwrap();
        Board {
            generation,
            rows,
            cols,
            grid,
        }
    }
    fn calc_neighbors(&self) -> Grid {
        let ref grid = self.grid;
        // make a border of dead cells, 0 as dead, 1 as alive
        let mut expanded_grid = Grid::from_elem(((self.rows + 2), (self.cols + 2)), 0);
        expanded_grid.slice_mut(s![1..-1,1..-1]).assign(&grid);

        let mut scratch = Grid::zeros((self.rows, self.cols));
        {
            // take eight directions as views,
            // add up all views into the scratch board,
            // then the value in every scratch board slot
            // is the number of the slot's alive neighbors
            let mut neighbor_view = scratch.view_mut();
            neighbor_view += &expanded_grid.slice(s![0..-2, 0..-2]); // north west
            neighbor_view += &expanded_grid.slice(s![0..-2, 1..-1]); // west
            neighbor_view += &expanded_grid.slice(s![0..-2, 2..  ]); // south west

            neighbor_view += &expanded_grid.slice(s![1..-1, 0..-2]); // north
            neighbor_view += &expanded_grid.slice(s![1..-1, 2..  ]); // south

            neighbor_view += &expanded_grid.slice(s![2.., 0..-2]); // north east
            neighbor_view += &expanded_grid.slice(s![2.., 1..-1]); // east
            neighbor_view += &expanded_grid.slice(s![2.., 2..  ]); // south east
        }
        scratch
    }
    pub fn iterate(&mut self) {
        let neighbor_grid = self.calc_neighbors();
        self.grid.zip_mut_with(&neighbor_grid, |y: &mut u8, &n: &u8| {
            *y = match (*y, n) {
                (_, 3) | (1, 2) => 1,
                _ => 0,
            }
        });
        // 2 or 3 neighbors: stay alive
        // 3 neighbors: birth
        // otherwise: death
        self.generation += 1;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let ref grid = self.grid;
        writeln!(f, "Generation {}:", self.generation)?;
        writeln!(f, "{} {}", self.rows, self.cols)?;
        for row in grid.genrows() {
            for &alive in row {
                if alive > 0 {
                    f.write_str("*")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}


// todo: add unit tests

#[cfg(test)]
mod tests {
    #[test]
    fn board_create() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn board_iterate() {
        assert_eq!(4, 2 + 2);
    }
}