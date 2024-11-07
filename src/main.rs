// use rand::prelude::*;

struct Grid {
  cells: Vec<Vec<Liveness>>,
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
    if self.cells[x+1][y] == Liveness::Alive {
      count += 1;
    }

    if self.cells[x-1][y] == Liveness::Alive {
      count += 1;
    }

    // x+1 , y
    // x-1 , y
    // x , y+1
    // x , y-1
    // x-1, y-1
    // x+1, y+1
    // x-1, y+1
    // y-1, x+1

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
    cells
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
}
