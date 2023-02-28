use druid::{
    widget::{Button, Flex, Padding},
    Widget,
};

use crate::AppState;

// struct Header;

pub fn build() -> impl Widget<AppState> {
    Padding::new(
        1.0,
        Flex::row()
            .with_child(Button::new("button1").on_click(|_, _, _| {
                // let mut log_info = self::AppState::log_info;
                let mut tmp = vec!["a".to_string()];
                tmp.push("b".to_string());
                for item in tmp {
                    println!("{}", item)
                }
                // self::AppState::line_number
            }))
            .with_child(Button::new("button2"))
            .with_child(Button::new("button3"))
            .with_child(Button::new("button4"))
            .with_child(Button::new("button5")),
    )
}
