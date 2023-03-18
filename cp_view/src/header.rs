use std::sync::Arc;

use cp_core::lexer;
use cp_core::relexer;
use druid::widget::Button;
use druid::{
    widget::{Flex, Padding},
    Widget,
};

use crate::AppState;

pub fn build() -> impl Widget<AppState> {
    Padding::new(
        1.0,
        Flex::row()
            .with_child(
                Button::new("词法分析").on_click(|_, data: &mut AppState, _| {
                    data.out_put = Arc::new(Vec::new());
                    data.log_info = Arc::new(Vec::new());

                    let mut cursor = lexer::Cursor::new(&data.source_code);
                    while !cursor.is_eof() {
                        match cursor.advance_token() {
                            Ok(token) => {
                                if token.not_whitespace() && token.not_comment() {
                                    Arc::make_mut(&mut data.out_put).push(format!("{}", token))
                                }
                            }
                            Err(token) => {
                                Arc::make_mut(&mut data.log_info).push(format!("Err: {}", token))
                            }
                        }
                    }
                }),
            )
            .with_child(
                Button::new("Re词法分析").on_click(|_, data: &mut AppState, _| {
                    data.out_put = Arc::new(Vec::new());
                    data.log_info = Arc::new(Vec::new());

                    let mut re_lexer = relexer::ReLexer::new(&data.source_code);
                    re_lexer.generate_token();
                    for token in re_lexer.tokens {
                        Arc::make_mut(&mut data.out_put).push(format!("{}", token))
                    }
                    for error in re_lexer.errors {
                        Arc::make_mut(&mut data.log_info).push(format!("Err: {}", error))
                    }
                }),
            )
            .with_child(Button::new("button3"))
            .with_child(Button::new("button4"))
            .with_child(Button::new("button5")),
    )
}
