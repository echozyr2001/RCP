use druid::{
    widget::{CrossAxisAlignment, Flex, Label, List, Scroll, Split},
    Widget, WidgetExt,
};

use crate::{AppState, FONT_SIZE, WINDOW_WIDTH};

pub fn build() -> impl Widget<AppState> {
    let output = Scroll::new(
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(make_header())
            .with_default_spacer()
            .with_flex_child(
                Scroll::new(List::new(make_list_item).lens(AppState::out_put)).vertical(),
                1.0,
            ),
    )
        .horizontal()
        .padding(10.0);
    let info = Scroll::new(
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(make_header())
            .with_default_spacer()
            .with_flex_child(
                Scroll::new(List::new(make_list_item).lens(AppState::log_info)).vertical(),
                1.0,
            ),
    )
        .horizontal()
        .padding(10.0);

    Split::rows(output, info)
}

fn make_header() -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Label::new("Row")
                .with_text_size(FONT_SIZE)
                .fix_width(WINDOW_WIDTH / 20.0),
        )
        .with_default_spacer()
        .with_child(
            Label::new("Column")
                .with_text_size(FONT_SIZE)
                .fix_width(WINDOW_WIDTH / 20.0),
        )
        .with_default_spacer()
        .with_child(
            Label::new("Value")
                .with_text_size(FONT_SIZE)
                .fix_width(WINDOW_WIDTH / 7.0),
        )
        .with_default_spacer()
        .with_child(
            Label::new("TokenKind")
                .with_text_size(FONT_SIZE)
                .fix_width(WINDOW_WIDTH / 5.0),
        )
}

fn make_list_item() -> impl Widget<String> {
    Flex::row()
        .with_child(
            Label::new(|a: &String, _env: &_| {
                format!("{}", a.split("@#").enumerate().nth(0).unwrap().1)
            })
                .with_text_size(FONT_SIZE)
                .fix_width(WINDOW_WIDTH / 20.0),
        )
        .with_default_spacer()
        .with_child(
            Label::new(|a: &String, _env: &_| {
                format!("{}", a.split("@#").enumerate().nth(1).unwrap().1)
            })
                .with_text_size(FONT_SIZE)
                .fix_width(WINDOW_WIDTH / 20.0),
        )
        .with_default_spacer()
        .with_child(
            Label::new(|a: &String, _env: &_| {
                format!("{}", a.split("@#").enumerate().nth(2).unwrap().1)
            })
                .with_text_size(FONT_SIZE)
                .fix_width(WINDOW_WIDTH / 7.0),
        )
        .with_default_spacer()
        .with_child(
            Label::new(|a: &String, _env: &_| {
                format!("{}", a.split("@#").enumerate().nth(3).unwrap().1)
            })
                .with_text_size(FONT_SIZE)
                .fix_width(WINDOW_WIDTH / 5.0),
        )
}
