struct Grid {
  cells: Vec<Vec<Liveness>>,
  size: (usize, usize)
}

impl Grid {
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
        (-1, -1), (0, -1), (-1, 1),
        (-1, 0),           (0, 1),
        (-1, 1),  (0, 1),  (1, 1)
        ];

    for (dx, dy) in &deltas {
        // Calculate the cell coords to look for
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        // Check if the cell coords are within bounds
        let x_valid = new_x >= 0 && new_x < self.size.0 as isize;
        let y_valid = new_y >= 0 && new_y < self.size.1 as isize;
        if x_valid && y_valid {
            // println!("Checking cell at ({}, {})", new_x, new_y);
            // Check if the cell is alive
            if self.cells[new_x as usize][new_y as usize] == Liveness::Alive {
                count += 1;
            }
        }
    }
    count
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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Liveness {
  Alive,
  Dead
}

fn main() {
  let g = random_grid(6,6);
  g.print();
  println!("Num alive neighbors: {}", g.count_alive_neighbors(1,1));
}
