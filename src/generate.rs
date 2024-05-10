use aglet::{Coord, Direction4, Direction4Set, Grid};
use fastrand::Rng;

use crate::board::{Board, Cell};
use crate::generate::Edge::{Lit, Unlit};

pub fn generate(w: u32, h: u32, seed: u64) -> Board {
  let mut rng = Rng::with_seed(seed);

  let mut vertical_edges = Grid::new(w + 1, h);
  let mut horizontal_edges = Grid::new(w, h + 1);

  (1..w).for_each(|x| {
    (0..h).for_each(|y| {
      vertical_edges.insert(Coord::new(x, y), generate_edge(&mut rng, x, y));
    })
  });

  (0..w).for_each(|x| {
    (1..h).for_each(|y| {
      horizontal_edges.insert(Coord::new(x, y), generate_edge(&mut rng, x, y));
    })
  });

  let mut grid = Grid::new(w, h);

  grid.area().into_iter().for_each(|Coord { x, y }| {
    let directions =
      get_single_direction_set(horizontal_edges.get_or_default(x, y), Direction4::North) |
        get_single_direction_set(horizontal_edges.get_or_default(x, y + 1), Direction4::South) |
        get_single_direction_set(vertical_edges.get_or_default(x, y), Direction4::West) |
        get_single_direction_set(vertical_edges.get_or_default(x + 1, y), Direction4::East);

    grid.insert(Coord::new(x, y), Cell::new(directions).spin(rng.i32(0..4)));
  });

  return Board::new(grid);
}

#[allow(unused_variables)]
fn generate_edge(rng: &mut Rng, x: u32, y: u32) -> Edge {
  return Edge::from(rng.bool())
}

fn get_single_direction_set(edge: Edge, direction: Direction4) -> Direction4Set {
  if edge == Lit {
    Direction4Set::from(direction)
  } else {
    Direction4Set::empty()
  }
}

trait GetOrDefaultByCoords<T> where T: Default, T: Copy {
  fn get_or_default(&self, x: u32, y: u32) -> T;
}

impl<T> GetOrDefaultByCoords<T> for Grid<T> where T: Default, T: Copy {
  fn get_or_default(&self, x: u32, y: u32) -> T {
    self.get(Coord::new(x, y)).copied().unwrap_or_default()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Edge {
  #[default]
  Unlit,
  Lit,
}

impl From<bool> for Edge {
  fn from(value: bool) -> Self {
    if value {
      Lit
    } else {
      Unlit
    }
  }
}