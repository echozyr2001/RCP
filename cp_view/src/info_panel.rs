use druid::{
    widget::{Label, List, Split},
    Env, Widget, WidgetExt,
};

use crate::{AppState, FONT_SIZE};

pub fn build() -> impl Widget<AppState> {
    let output = Label::new(|data: &AppState, _: &Env| format!("Line {}", data.line_number))
        .with_text_size(FONT_SIZE);

    let log = List::new(make_list).lens(AppState::log_info);
    Split::rows(output, log)
}

fn make_list() -> impl Widget<String> {
    Label::dynamic(|s: &String, _| s.to_string())
}
