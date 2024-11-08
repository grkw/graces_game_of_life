struct Grid {
  cells: Vec<Vec<Liveness>>,
  size: (usize, usize)
}

impl Grid {

    fn new(height: usize, width: usize) -> Self {
        let mut grid = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Liveness::Dead);
            }
            grid.push(row);
        }
        Grid {
            cells: grid,
            size: (height, width)
        }
    }

    fn random_grid(width: usize, height: usize) -> Grid {
      let mut cells = Vec::with_capacity(height);
      for j in 0..height {
        let mut vec = Vec::with_capacity(width);
        for i in 0..width {
          if rand::random() {
            vec.insert(i, Liveness::Alive);
          } else {
            vec.insert(i, Liveness::Dead);
          }
        }
        cells.insert(j, vec);
      }
      Grid {
        cells: cells,
        size: (width, height)
      }
    }

  fn print(&self) {
    for row in &self.cells {
      for cell in row {
        match cell {
            Liveness::Alive => {
              print!(" X ");
            }
            Liveness::Dead => {
              print!(" . ");
            }
        }
      }
      println!();
    }
  }

  fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
    let mut count = 0;

    if x >= self.size.0 || y >= self.size.1 {
        panic!("Cell of interest is out of bounds");
    }

    let deltas = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (0, 1),
        (-1, 1),  (1, 0),  (1, 1)
        ];

    for (dx, dy) in &deltas {
        // Calculate the cell coords to look for
        let new_x = x as isize + dx; // Convert to isize to allow for negative values
        let new_y = y as isize + dy;

        // Check if the cell coords are within bounds
        let x_valid = new_x >= 0 && new_x < self.size.0 as isize;
        let y_valid = new_y >= 0 && new_y < self.size.1 as isize;
        if x_valid && y_valid {
            // println!("Checking cell at ({}, {})", new_x, new_y);
            // Check if the cell is alive
            if self.cells[new_x as usize][new_y as usize] == Liveness::Alive {
                // println!("alive!");
                count += 1;
            }
        }
    }
    count
  }

  fn update(&self) -> Grid {
      let height = self.size.0;
      let width = self.size.1;

      let mut g = Grid::new(height, width);

      for r in 0..height {
          for c in 0..width {
              let count = self.count_alive_neighbors(r, c);
              // println!("count_alive_neighbors({},{}): {}", r, c, count);
              if self.cells[r][c] == Liveness::Alive {
                  // println!("{},{} is alive", r, c);
                  if count == 2 || count == 3 { // Default is dead
                      g.cells[r][c] = Liveness::Alive;
                  }
              } else {
                  // println!("{},{} is dead", r, c);
                    if count == 3 {
                        g.cells[r][c] = Liveness::Alive;
                    }
              }
          }
      }

      g
  }

}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Liveness {
  Alive,
  Dead
}

fn main() {
  println!("Welcome to Grace's Game of Life");
  let g = Grid::random_grid(3,3);
  g.print();
  println!("");
  let new_g = g.update();
  new_g.print();
}
