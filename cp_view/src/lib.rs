use std::sync::Arc;

use druid::{
    widget::{Padding, Split},
    AppLauncher, Data, Lens, Widget, WindowDesc,
};

const WINDOW_HEIGHT: f64 = 720.0;
const WINDOW_WIDTH: f64 = 1280.0;
const MIN_SIZE: f64 = 500.0;
const FONT_SIZE: f64 = 30.0;

#[derive(Data, Lens, Clone)]
pub struct AppState {
    source_text: String,
    line_number: usize,
    log_info: Arc<Vec<String>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            source_text: format!("this is source_text"),
            line_number: 1,
            log_info: Arc::new(Vec::new()),
        }
    }
}

mod edit_panel;
mod header;
mod info_panel;

pub fn show() {
    let main_window = WindowDesc::new(ui_builder())
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_min_size((MIN_SIZE, MIN_SIZE))
        .title(format!("My Complier"));

    AppLauncher::with_window(main_window)
        .launch(AppState::new())
        .expect("Launch Window Error!");
}

fn ui_builder() -> impl Widget<AppState> {
    let header = header::build();
    let body = Split::columns(edit_panel::build(), info_panel::build());

    Padding::new(1.0, Split::rows(header, body).split_point(0.1))
}
