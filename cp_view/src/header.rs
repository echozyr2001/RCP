use std::sync::Arc;

use cp_core::lexer::Cursor;
use cp_core::relexer;
use cp_core::parser;
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

          let mut cursor = Cursor::new(&data.source_code);
          while !cursor.is_eof() {
            match cursor.advance_token() {
              Ok(token) => {
                if !token.is_whitespace() && !token.is_comment() {
                  // Arc::make_mut(&mut data.out_put).push(format!("{}", token))
                  println!("{}", token);
                }
              }
              Err(err) => {
                // Arc::make_mut(&mut data.log_info).push(format!("{}", err.message))
                println!("{}", err.message);
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
            Arc::make_mut(&mut data.log_info).push(format!("{}", error))
          }
        }),
      )
      .with_child(Button::new("语法分析").on_click(|_, data: &mut AppState, _| {
        let mut cursor = Cursor::new(&data.source_code);
        let mut input = vec![];
        while !cursor.is_eof() {
          if let Ok(token) = cursor.advance_token() {
            if !token.is_whitespace() && !token.is_comment() {
              input.push(token);
            }
          }
        }

        let mut grammar = parser::Grammar::new();
        grammar.grammar_load("./g3.txt");

        let mut lr1 = parser::LR1Parser::new();
        lr1.compute_lr1_item_sets(&grammar);
        lr1.construct_parsing_table(&grammar);

        for action in &lr1.action_table {
          println!("{:?}", action);
        }
        println!(" ");
        for goto in &lr1.goto_table {
          println!("{:?}", goto);
        }

        let tmp = lr1.construct_tree(&input);
        println!("{}", tmp);
      }))
      .with_child(Button::new("button4"))
      .with_child(Button::new("button5")),
  )
}
