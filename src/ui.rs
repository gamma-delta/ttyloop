use aglet::{Coord, Direction4, Rotation};
use cursive::{
  direction::Direction,
  event::{Event, EventResult, Key},
  theme::{
    BaseColor, BorderStyle, Color, ColorStyle, Effect, Palette, PaletteStyle,
    Style, Theme,
  },
  view::{CannotFocus, ViewWrapper},
  views::{
    Button, Dialog, LinearLayout, NamedView, Panel, SliderView, TextView,
  },
  Cursive, Printer, Vec2, View, With,
};

use crate::board::{Board, Cell};

pub struct HjklToDirectionWrapperView<T> {
  view: T,
}

impl<T> HjklToDirectionWrapperView<T> {
  pub fn new(view: T) -> Self {
    Self { view }
  }

  cursive::inner_getters!(self.view: T);
}

impl<T> ViewWrapper for HjklToDirectionWrapperView<T>
where
  T: View,
{
  cursive::wrap_impl!(self.view: T);

  fn wrap_on_event(&mut self, ev: Event) -> EventResult {
    let ev_result = self.view.on_event(ev.clone());
    if let (
      EventResult::Ignored,
      Event::Char('h') | Event::Char('j') | Event::Char('k') | Event::Char('l'),
    ) = (&ev_result, &ev)
    {
      let dir = match ev {
        Event::Char('h') => Key::Left,
        Event::Char('j') => Key::Down,
        Event::Char('k') => Key::Up,
        Event::Char('l') => Key::Right,
        _ => unreachable!(),
      };
      self.view.on_event(Event::Key(dir))
    } else {
      ev_result
    }
  }
}

pub struct BoardView {
  board: Board,
  focused: Coord,
  solved: bool,
}

impl BoardView {
  pub fn new(board: Board) -> BoardView {
    BoardView {
      board,
      focused: Coord::ZERO,
      solved: false,
    }
  }

  pub fn clobber_board(&mut self, new_board: Board) {
    self.solved = false;
    // if !new_board.inner.area().contains(self.focused) {
    //   self.focused = Coord::ZERO;
    // }
    self.board = new_board;
  }

  pub fn board(&self) -> &Board {
    &self.board
  }
}

impl View for BoardView {
  fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
    Vec2::new(
      self.board.inner.width() as usize * Cell::DISPLAY_LEN,
      self.board.inner.height() as usize,
    )
  }

  fn draw(&self, printer: &Printer) {
    for (pos, cell) in self.board.inner.iter() {
      // i've decided the variant looks weird
      let text = cell.render(false);

      let fg = Color::RgbLowRes(5, 5, 5);

      let bg = if self.solved {
        Color::TerminalDefault
      } else {
        let checkerboard = (pos.x + pos.y) % 2 == 0;
        let focus_here = self.focused == pos;
        match (focus_here, checkerboard) {
          (false, false) => Color::RgbLowRes(1, 1, 1),
          (true, false) => Color::RgbLowRes(1, 1, 3),
          (false, true) => Color::RgbLowRes(2, 2, 2),
          (true, true) => Color::RgbLowRes(2, 2, 4),
        }
      };

      printer.with_color(ColorStyle::new(fg, bg), |p| {
        p.with_effect(Effect::Bold, |p| {
          p.print((pos.x * Cell::DISPLAY_LEN as u32, pos.y), text)
        })
      });
    }
  }

  fn take_focus(
    &mut self,
    _source: Direction,
  ) -> Result<EventResult, CannotFocus> {
    Ok(EventResult::Consumed(None))
  }

  fn on_event(&mut self, ev: Event) -> EventResult {
    match ev {
      Event::Char(ch) => match ch {
        _ if "ui".contains(ch) && !self.solved => {
          let rot = match ch {
            'u' => Rotation::CounterClockwise,
            'i' => Rotation::Clockwise,
            _ => unreachable!(),
          };
          self.board.rotate_cell(self.focused, rot);

          if self.board.check_ok() {
            self.solved = true;
          }
          EventResult::consumed()
        }

        _ => EventResult::Ignored,
      },
      Event::Key(key @ (Key::Up | Key::Down | Key::Left | Key::Right))
        if !self.solved =>
      {
        let delta = match key {
          Key::Left => Direction4::West,
          Key::Down => Direction4::South,
          Key::Up => Direction4::North,
          Key::Right => Direction4::East,
          _ => unreachable!(),
        };
        let w = self.board.inner.width();
        let h = self.board.inner.height();
        let ipos = self.focused.to_icoord() + delta.deltas();
        let x = ipos.x.rem_euclid(w as _) as u32;
        let y = ipos.y.rem_euclid(h as _) as u32;
        self.focused = Coord::new(x, y);
        EventResult::consumed()
      }

      _ => EventResult::Ignored,
    }
  }
}

/// Min and max are inclusive
#[derive(Debug)]
pub struct TheCoolerSliderView {
  title: String,
  min: u32,
  max: u32,
  val_offset: u32,
}

impl TheCoolerSliderView {
  pub fn new<S: Into<String>>(
    title: S,
    min: u32,
    max: u32,
    start: u32,
  ) -> Self {
    if !(max >= min) {
      panic!(
        "the cooler slider view max {} must be >= the min {}",
        max, min
      )
    }
    let start = start.clamp(min, max);
    Self {
      title: title.into(),
      min,
      max,
      val_offset: start - min,
    }
  }

  fn span_size(&self) -> u32 {
    self.max + 1 - self.min
  }

  fn side_char_sizes(&self) -> (u32, u32) {
    let lhs_chars = self.min.checked_ilog10().unwrap_or(0) + 1;
    let rhs_chars = self.max.checked_ilog10().unwrap_or(0) + 1;
    (lhs_chars, rhs_chars)
  }

  pub fn value(&self) -> u32 {
    self.min + self.val_offset
  }
}

impl View for TheCoolerSliderView {
  fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
    let (lhs_chars, rhs_chars) = self.side_char_sizes();
    let size = lhs_chars + 1 + self.span_size() + 1 + rhs_chars;
    Vec2::new(size as usize, 2)
  }

  fn draw(&self, printer: &Printer) {
    let msg = format!("{}: {}", self.title, self.value());
    printer.print(Vec2::zero(), &msg);

    let (lhs_char_sz, _) = self.side_char_sizes();
    printer.print((0, 1), &self.min.to_string());
    printer.print_hline((lhs_char_sz + 1, 1), self.span_size() as usize, "-");
    printer.print(
      (lhs_char_sz + 1 + self.span_size() + 1, 1),
      &self.max.to_string(),
    );

    let style = if printer.focused {
      PaletteStyle::Highlight
    } else {
      PaletteStyle::HighlightInactive
    };
    printer.with_style(style, |printer| {
      printer.print((lhs_char_sz + 1 + self.val_offset, 1), "#");
    });
  }

  fn on_event(&mut self, ev: Event) -> EventResult {
    match ev {
      Event::Key(Key::Left) => {
        if self.val_offset > 0 {
          self.val_offset -= 1;
        }
        EventResult::consumed()
      }
      Event::Key(Key::Right) => {
        if self.val_offset < self.span_size() - 1 {
          self.val_offset += 1;
        }
        EventResult::consumed()
      }
      Event::Ctrl(Key::Left) | Event::Shift(Key::Left) => {
        self.val_offset = 0;
        EventResult::consumed()
      }
      Event::Ctrl(Key::Right) | Event::Shift(Key::Right) => {
        self.val_offset = self.span_size() - 1;
        EventResult::consumed()
      }
      _ => EventResult::Ignored,
    }
  }

  fn take_focus(
    &mut self,
    _source: Direction,
  ) -> Result<EventResult, CannotFocus> {
    Ok(EventResult::consumed())
  }
}

pub fn cool_theme() -> Theme {
  // https://github.com/gyscos/cursive/blob/main/cursive/examples/theme_manual.rs
  cursive::theme::Theme {
    shadow: true,
    borders: BorderStyle::Simple,
    palette: Palette::retro().with(|palette| {
      use cursive::theme::BaseColor::*;

      {
        // First, override some colors from the base palette.
        use cursive::theme::{Color::TerminalDefault, PaletteColor::*};

        palette[Background] = TerminalDefault;
        palette[View] = TerminalDefault;
        palette[Primary] = White.dark();
        palette[TitlePrimary] = Blue.light();
        palette[Secondary] = Blue.light();
        palette[Highlight] = Cyan.light();
        palette[HighlightInactive] = Blue.dark();
      }
    }),
  }
}
