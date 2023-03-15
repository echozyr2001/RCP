use druid::{
    widget::{Label, List, Scroll, Split},
    Widget, WidgetExt,
};

use crate::{AppState, FONT_SIZE};

pub fn build() -> impl Widget<AppState> {
    let output = Scroll::new(List::new(make_list).lens(AppState::out_put));

    let log = Scroll::new(List::new(make_list).lens(AppState::log_info));
    Split::rows(output, log)
}

fn make_list() -> impl Widget<String> {
    Label::dynamic(|s: &String, _| s.to_string()).with_text_size(FONT_SIZE)
}
