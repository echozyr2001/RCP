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
            .with_child(Button::new("button1").on_click(|_, _data, _| {
                // let mut log_info = self::AppState::log_info;
                let mut tmp = vec!["a".to_string()];
                tmp.push("b".to_string());
                for item in tmp {
                    println!("{}", item)
                }
                // self::AppState::line_number
            }))
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
            .with_child(Button::new("button2"))
            .with_child(Button::new("button3"))
            .with_child(Button::new("button4"))
            .with_child(Button::new("button5")),
    )
}
