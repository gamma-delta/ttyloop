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

  (0..w).for_each(|x| {
    (0..h).for_each(|y| {
      let directions =
          get_single_direction_set(&horizontal_edges, Direction4::North, x, y) |
              get_single_direction_set(&horizontal_edges, Direction4::South, x, y + 1) |
              get_single_direction_set(&vertical_edges, Direction4::West, x, y) |
              get_single_direction_set(&vertical_edges, Direction4::East, x + 1, y);

      grid.insert(Coord::new(x, y), Cell::new(directions).spin(rng.i32(0..4)));
    })
  });

  return Board::new(grid);
}

#[allow(unused_variables)]
fn generate_edge(rng: &mut Rng, x: u32, y: u32) -> Edge {
  return Edge::from(rng.bool())
}

fn get_single_direction_set(edges: &Grid<Edge>, direction: Direction4, x: u32, y: u32) -> Direction4Set {
  if get_edge(edges, x, y) == Lit {
    Direction4Set::from(direction)
  } else {
    Direction4Set::empty()
  }
}

fn get_edge(edges: &Grid<Edge>, x: u32, y: u32) -> Edge {
  edges.get(Coord::new(x, y)).copied().unwrap_or_default()
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

