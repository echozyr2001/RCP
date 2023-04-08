mod grammar;
mod lr1_parser;

use lazy_static::lazy_static;
use crate::token::{Token, TokenType, TokenKind};
pub use grammar::Grammar;
pub use lr1_parser::{LR1Parser, TreeNode};
use crate::lexer::Point;

const DATA_PATH: &str = "./data/";
const ACTION_TABLE: &str = "./data/action_table.rcp";
const GOTO_TABLE: &str = "./data/goto_table.rcp";
const LR1_SETS: &str = "./data/lr1_sets.rcp";

lazy_static!(
  pub static ref START_SYMBOL: Token = Token::new_not_terminal("CompUnit'".to_string(), None);
  // pub static ref END_SYMBOL: Token = Token::new_terminal("#".to_string(), None);
  pub static ref END_SYMBOL: Token = Token::new(TokenType::from("#"), "#".to_string(), Some(Point::new(0, 0)));
  pub static ref EMPTY_SYMBOL: Token = Token::new_terminal("ε".to_string(), None);
  pub static ref ERROR_SYMBOL: Token = Token::new_terminal("err".to_string(), None);
);

// 单元测试
#[cfg(test)]
mod tests {
  use super::*;

  // 打印grammar
  #[test]
  fn test_grammar() {
    let mut grammar = Grammar::new();
    grammar.grammar_load("../g.txt");
    println!("{:#?}", grammar);
  }

  // 打印lr1项目集
  #[test]
  fn test_lr1_sets() {
    let mut grammar = Grammar::new();
    grammar.grammar_load("../g.txt");
    let mut lr1_parser = LR1Parser::new();
    lr1_parser.compute_lr1_item_sets(&grammar);
    println!("{:?}", lr1_parser.lr1_sets);
  }

  // 打印lr1分析表
  #[test]
  fn test_lr1_table() {
    let mut grammar = Grammar::new();
    grammar.grammar_load("../g.txt");
    let mut lr1_parser = LR1Parser::new();
    lr1_parser.compute_lr1_item_sets(&grammar);
    lr1_parser.construct_parsing_table(&grammar);
    println!("{:?}", lr1_parser.action_table);
    println!("{:?}", lr1_parser.goto_table);
  }
}