use cursive::{
  views::{
    Button, Dialog, Layer, LinearLayout, NamedView, Panel, SliderView, TextView,
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
          TheCoolerSliderView::new("Width", 5, 20),
        ))
        .child(NamedView::new(
          "height",
          TheCoolerSliderView::new("Height", 5, 20),
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

  siv.add_layer(hjkl(
    Dialog::new().title("ttyloop").content(
      LinearLayout::vertical()
        .child(Panel::new(BoardView::new(w, h, None)))
        .child(TextView::new(
          "hjkl: move\n\
          ui: rotate\n\
          n: new puzzle\n\
          c: configure\n\
          q: quit",
        )),
    ),
  ))
}

fn hjkl<V: View>(view: V) -> HjklToDirectionWrapperView<V> {
  HjklToDirectionWrapperView::new(view)
}
