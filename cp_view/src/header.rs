use std::sync::Arc;

use druid::widget::{Button, Label};
use druid::{
    widget::{Flex, Padding},
    Widget,
};
use druid::{LocalizedString, WidgetExt};
use opener::open_browser;

use crate::AppState;

pub fn build() -> impl Widget<AppState> {
    Padding::new(
        1.0,
        Flex::row()
            .with_child(
                Button::new("词法分析").on_click(|_, data: &mut AppState, _| {
                    // let mut log_info = self::AppState::log_info;
                    // let mut a = Vec::<String>::new();
                    // a.push("s".to_string());

                    // _data.log_info.lock().unwrap().push("aaa".to_string());

                    // self::AppState::line_number

                    Arc::make_mut(&mut data.log_info).push("sss".to_string());
                }),
            )
            .with_child(
                Label::new(LocalizedString::new("Click here to visit Example.com")).on_click(
                    |_, _, _| {
                        open_browser(
                            "https://echo-zyr-2001s-organization.gitbook.io/rust_complier/",
                        )
                        .unwrap();
                    },
                ),
            )
            .with_child(
                Button::new("button2").on_click(|_, data: &mut AppState, _| {
                    println!("line number is {}", cp_core::count_line(&data.source_text));
                }),
            )
            .with_child(Button::new("button3"))
            .with_child(Button::new("button4"))
            .with_child(Button::new("button5")),
    )
}
