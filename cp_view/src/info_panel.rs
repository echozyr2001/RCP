use druid::{
    widget::{Label, Split},
    Env, Widget,
};

use crate::{AppState, FONT_SIZE};

pub fn build() -> impl Widget<AppState> {
    let output = Label::new(|data: &AppState, _: &Env| format!("Line {}", data.line_number))
        .with_text_size(FONT_SIZE);
    let log = Label::new(format!("this is log")).with_text_size(FONT_SIZE);
    Split::rows(output, log)
}
