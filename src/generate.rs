use aglet::{Area, Coord, Direction4, Direction4Set, Grid};
use fastrand::Rng;
use itertools::Itertools;

use crate::board::{Board, Cell};

pub fn generate(w: u32, h: u32, seed: u64) -> Board {
  for seed_offset in 0u64.. {
    let board_mb = generate_inner(w, h, seed + seed_offset);
    match board_mb {
      Ok(board) => return board,
      Err(ono) => eprintln!("{}", ono),
    }
  }
  unreachable!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Edge {
  #[default]
  Unknown,
  Unlit,
  Lit,
}

impl Edge {
  fn valid_fits(other: Option<Edge>) -> &'static [Edge] {
    match other {
      Some(Edge::Unknown) => &[Edge::Unlit, Edge::Lit],
      Some(Edge::Lit) => &[Edge::Lit],
      Some(Edge::Unlit) => &[Edge::Unlit],
      // poking out of bounds
      None => &[Edge::Unlit],
    }
  }
}

#[derive(Debug, Clone, Copy, Default)]
struct GenCell {
  edges: [Edge; 4],
}

struct WfcBoard {
  grid: Grid<GenCell>,
  rng: Rng,
}

impl WfcBoard {
  fn new(w: u32, h: u32, seed: u64) -> Self {
    let rng = Rng::with_seed(seed);
    let mut grid = Grid::new(w, h);
    for x in 0..w {
      for y in 0..h {
        grid.insert(Coord::new(x, y), GenCell::default());
      }
    }

    WfcBoard { grid, rng }
  }

  fn collapse(&mut self, pos: Coord) -> bool {
    let here = *self.grid.get_or_insert(pos, GenCell::default());

    let picked_edges = Direction4::DIRECTIONS
      .into_iter()
      .map(|dir| {
        let edge = here.edges[dir.ordinal()];
        if edge != Edge::Unknown {
          return edge;
        }
        let neighbor_edge = pos
          .offset4(dir)
          .and_then(|npos| self.grid.get(npos))
          .map(|cell| cell.edges[dir.flip().ordinal()]);
        let ok_edges = Edge::valid_fits(neighbor_edge);
        // Here is where the collapse happens
        ok_edges[self.rng.usize(0..ok_edges.len())]
      })
      .collect_vec();
    let picked_edges: [Edge; 4] = picked_edges.try_into().unwrap();
    debug_assert!(!picked_edges.contains(&Edge::Unknown));

    let cell = GenCell {
      edges: picked_edges,
    };
    self.grid.insert(pos, cell);
    true
  }
}

fn generate_inner(w: u32, h: u32, seed: u64) -> Result<Board, String> {
  let mut wfc = WfcBoard::new(w, h, seed);
  let to_collapse = {
    let mut cells = Area::new(Coord::ZERO, w, h).into_iter().collect_vec();
    wfc.rng.shuffle(&mut cells);
    cells
  };

  for &coord in to_collapse.iter() {
    wfc.collapse(coord);
  }

  let mut out = Grid::<Cell>::new(w, h);
  // might as well reuse the old vec
  for &coord in to_collapse.iter() {
    let wfc_cell = wfc
      .grid
      .get(coord)
      .ok_or_else(|| format!("had no wfc cell at all at {}", coord))?;

    let mut dirs = Direction4Set::empty();
    for dir in Direction4::DIRECTIONS {
      let wfc_edge = wfc_cell.edges[dir.ordinal()];
      let edge_on = match wfc_edge {
        Edge::Unlit => false,
        Edge::Lit => true,
        Edge::Unknown => {
          return Err(format!("found uncollapsed edge at {}, {:?}", coord, dir))
        }
      };
      if edge_on {
        dirs |= dir;
      }
    }

    let cell = Cell::new(dirs);
    let cell = cell.spin(wfc.rng.i32(0..=3));
    out.insert(coord, cell);
  }

  Ok(Board::new(out))
}
