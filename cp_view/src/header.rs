use druid::{
    widget::{Button, Flex, Padding},
    Widget,
};

struct Header;

pub fn build() -> impl Widget<()> {
    Padding::new(
        1.0,
        Flex::column()
            .with_child(Button::new("button1"))
            .with_child(Button::new("button2")),
    )
}
