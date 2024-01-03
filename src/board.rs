use aglet::{Coord, Direction4, Direction4Set, Grid, Rotation};

#[derive(Debug, Clone, Copy, Default)]
pub struct Cell {
  inner: Direction4Set,
  // used for the cute little 4-way rotating variant thing
  variant: bool,
}

impl Cell {
  pub const DISPLAY_LEN: usize = 3;

  pub fn new(inner: Direction4Set) -> Self {
    Self {
      inner,
      variant: false,
    }
  }

  pub fn rotated(self, rot: Rotation) -> Self {
    self.spin(rot.steps_clockwise())
  }

  pub fn spin(self, rot: i32) -> Self {
    // TODO: does rustc know how to optimize this. probably
    let mut out = Direction4Set::empty();
    for dir in self.inner.into_iter() {
      out |= dir.rotate_by(rot);
    }
    Self {
      inner: out,
      variant: !self.variant,
    }
  }

  pub fn fits_with(self, other: Option<Cell>, dir: Direction4) -> bool {
    let my_light = self.inner.contains(Direction4Set::from_flag(dir));
    let their_light = other
      .map(|other| other.inner.contains(Direction4Set::from_flag(dir.flip())))
      .unwrap_or(false);
    my_light == their_light
  }

  pub fn render(self, variant_4: bool) -> &'static str {
    let sides = self.inner.bits();
    match sides {
      0b0000 => "   ",

      // One
      0b0001 => " ╵ ",
      0b0010 => " ╶─",
      0b0100 => " ╷ ",
      0b1000 => "─╴ ",

      // Two, corners
      0b0011 => " ╰─",
      0b0110 => " ╭─",
      0b1100 => "─╮ ",
      0b1001 => "─╯ ",
      // Two,  straights
      0b0101 => " │ ",
      0b1010 => "───",

      // Three
      0b0111 => " ├─",
      0b1110 => "─┬─",
      0b1101 => "─┤ ",
      0b1011 => "─┴─",

      // Four
      0b1111 => {
        if variant_4 {
          if self.variant {
            "─🮨─"
          } else {
            "─🮩─"
          }
        } else {
          "─┼─"
        }
      }

      // make sure i caught all 15 by using the range
      16.. => unreachable!(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Board {
  pub inner: Grid<Cell>,
}

impl Board {
  pub fn new(inner: Grid<Cell>) -> Self {
    Self { inner }
  }

  pub fn check_ok(&self) -> bool {
    for (pos, cell) in self.inner.iter() {
      // make sure i check that the top and left doesn't
      // poke out of the board
      let dirs = if pos.x == 0 || pos.y == 0 {
        Direction4::DIRECTIONS.as_slice()
      } else {
        [Direction4::East, Direction4::South].as_slice()
      };
      for &dir in dirs {
        let neighbor = pos
          .offset4(dir)
          .and_then(|npos| self.inner.get(npos))
          .copied();
        if !cell.fits_with(neighbor, dir) {
          return false;
        }
      }
    }
    true
  }

  pub fn check_single_cell_ok(&self, pos: Coord) -> bool {
    let Some(&here) = self.inner.get(pos) else {
      return true;
    };
    for dir in Direction4::DIRECTIONS {
      let neighbor = pos
        .offset4(dir)
        .and_then(|npos| self.inner.get(npos))
        .copied();
      let my_light = here.inner.contains(Direction4Set::from_flag(dir));
      let their_light = neighbor
        .map(|neighbor| {
          neighbor
            .inner
            .contains(Direction4Set::from_flag(dir.flip()))
        })
        .unwrap_or(false);
      let fits = (!my_light) || (my_light && their_light);
      if !fits {
        return false;
      }
    }

    true
  }

  pub fn get_or_default(&mut self, coord: Coord) -> &mut Cell {
    self.inner.get_or_insert(coord, Cell::default())
  }

  pub fn rotate_cell(&mut self, coord: Coord, rot: Rotation) {
    let c = self.get_or_default(coord);
    *c = c.rotated(rot);
  }
}
