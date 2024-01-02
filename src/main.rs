use cursive::{
  views::{
    Button, Dialog, LinearLayout, NamedView, OnEventView, Panel, TextView,
  },
  Cursive, View,
};
use ui::{BoardView, HjklToDirectionWrapperView, TheCoolerSliderView};

mod board;
mod generate;
mod ui;

fn main() {
  let mut siv = cursive::default();
  siv.set_theme(ui::cool_theme());

  siv.add_layer(hjkl(
    Dialog::new().title("ttyloop").content(
      LinearLayout::vertical()
        .child(NamedView::new(
          "width",
          TheCoolerSliderView::new("Width", 5, 20, 9),
        ))
        .child(NamedView::new(
          "height",
          TheCoolerSliderView::new("Height", 5, 20, 9),
        ))
        .child(Button::new("Play", on_play)),
    ),
  ));

  siv.run();
}

fn on_play(siv: &mut Cursive) {
  let w = siv
    .find_name::<TheCoolerSliderView>("width")
    .unwrap()
    .value();
  let h = siv
    .find_name::<TheCoolerSliderView>("height")
    .unwrap()
    .value();

  let board = generate::generate(w, h, fastrand::u64(..));

  siv.add_layer(
    OnEventView::new(hjkl(
      Dialog::new().title("ttyloop").content(
        LinearLayout::vertical()
          .child(Panel::new(NamedView::new("board", BoardView::new(board))))
          .child(TextView::new(
            "hjkl/arrows: move\n\
          ui: rotate\n\
          n: new puzzle\n\
          c: configure\n\
          ctrl+C: quit",
          )),
      ),
    ))
    // the board handles hjlk ui
    // .on_event('q', |s| {
    //   s.quit();
    // })
    .on_event('c', |s| {
      s.pop_layer();
    })
    .on_event('n', |s| {
      let mut board_view = s.find_name::<BoardView>("board").unwrap();
      let area = board_view.board().inner.area();

      let new_board =
        generate::generate(area.width, area.height, fastrand::u64(..));
      board_view.clobber_board(new_board);
    }),
  )
}

fn hjkl<V: View>(view: V) -> HjklToDirectionWrapperView<V> {
  HjklToDirectionWrapperView::new(view)
}
