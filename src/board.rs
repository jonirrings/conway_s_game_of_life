use std::fmt::{self, Display, Formatter};

use ndarray::Array;
use ndarray::Array2;

type Grid = Array2<u8>;

pub struct Board {
    generation: usize,
    rows: usize,
    cols: usize,
    grid: Grid,
    scratch: Grid,
}

impl Board {
    pub fn new(rows: usize, cols: usize, cells: Vec<u8>, generation: usize) -> Board {
        // make a border of dead cells, 0 as dead, 1 as alive
        let mut grid = Grid::from_elem(((rows + 2), (cols + 2)), 0);
        let scratch = Grid::zeros((rows, cols));
        let a = Array::from_vec(cells).into_shape((rows, cols)).unwrap();
        grid.slice_mut(s![1..-1, 1..-1]).assign(&a);
        Board {
            generation,
            rows,
            cols,
            grid,
            scratch,
        }
    }
    pub fn iterate(&mut self) {
        let ref mut grid = self.grid;
        let ref mut scratch = self.scratch;
        let mut neigh = scratch.view_mut();

        neigh.fill(0);
        // take eight directions as views,
        // add up all views in a scratch board,
        // then the value in every scratch board slot is the number of its alive neighbors
        neigh += &grid.slice(s![0..-2, 0..-2]); // north west
        neigh += &grid.slice(s![0..-2, 1..-1]); // west
        neigh += &grid.slice(s![0..-2, 2..]); // south west

        neigh += &grid.slice(s![1..-1, 0..-2]); // north
        neigh += &grid.slice(s![1..-1, 2..]); // south

        neigh += &grid.slice(s![2.., 0..-2]); // north east
        neigh += &grid.slice(s![2.., 1..-1]); // east
        neigh += &grid.slice(s![2.., 2..]); // south east

        let mut grid_view = grid.slice_mut(s![1..-1, 1..-1]); // central grid view

        // 2 or 3 neighbors: stay alive
        // 3 neighbors: birth
        // otherwise: death
        grid_view.zip_mut_with(&neigh, |y: &mut u8, &n: &u8| {
            *y = match (*y, n) {
                (_, 3) | (1, 2) => 1,
                (_, _) => 0,
            }
        });
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