use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use crate::parser::{EMPTY_SYMBOL, START_SYMBOL};
use crate::token::{TokenKind, TokenType, Token};

pub type Item = Vec<String>;
pub type PHead = String;
pub type PBody = Vec<Item>;

#[derive(Debug)]
pub struct Grammar {
  file_buff: String,
  pub(crate) token_list: Vec<String>,
  pub(crate) pro_list: BTreeMap<PHead, PBody>,
  pub(crate) first_sets: BTreeMap<String, BTreeSet<String>>,
  pub(crate) start_symbol: String,
}

impl Grammar {
  pub fn new() -> Self {
    Self {
      token_list: Vec::<String>::new(),
      pro_list: BTreeMap::<PHead, PBody>::new(),
      first_sets: BTreeMap::<String, BTreeSet<String>>::new(),
      file_buff: String::new(),
      start_symbol: START_SYMBOL.get_value().to_string(),
    }
    // Self::default()
  }

  fn file_load(&mut self, file_path: &str) {
    let mut file = File::open(file_path).expect("File open Err!!{}");
    file.read_to_string(&mut self.file_buff).expect("File read Err!!");
  }

  pub fn grammar_load(&mut self, grammar_path: &str) {
    self.file_load(grammar_path);
    let lines = self.file_buff.lines();
    for line in lines {
      if line.starts_with('{') { continue; }
      if line.starts_with('}') { continue; }

      if line.starts_with("%token") {
        let tokens = line.split_whitespace().skip(1);
        for token in tokens {
          self.token_list.push(token.to_string());
        }
        continue;
      }
      {
        let mut tmp = line.split(':'); // 只有可能是两部分
        // let p_head = PHead::NotTerminal(tmp.next().unwrap().to_string());
        // let p_head = Token::new_not_terminal(tmp.next().unwrap().to_string(), None);
        // let p_head = TokenType::from(tmp.next().unwrap());
        let p_head = tmp.next().unwrap().to_string();
        let mut p_body = PBody::new();
        let items = tmp.next().unwrap().split(" | "); // 拆分右部
        for item in items {
          let elements = item.split_whitespace();
          let mut item = Item::new();
          for element in elements {
            item.push(
              if self.token_list.contains(&element.to_string()) {
                // Element::Terminal(element.to_string())
                // Token::new_terminal(element.to_string(), None)
                // TokenType::from(element)
                element.to_string()
              } else {
                // Element::NotTerminal(element.to_string())
                // Token::new_not_terminal(element.to_string(), None)
                // TokenType::from(element)
                element.to_string()
              }
            );
          }
          p_body.push(item);
        }
        self.pro_list.insert(p_head, p_body);
      }
    }

    self.calculate_first_sets();
  }

  fn first(&mut self, symbol: &str) -> BTreeSet<String> {
    if let Some(first_set) = self.first_sets.get(symbol) {
      return first_set.clone();
    }

    let mut result = BTreeSet::new();
    if self.token_list.contains(&symbol.to_string()) {
      result.insert(symbol.to_string());
    } else {
      if let Some(productions) = self.pro_list.get(symbol) {
        let productions: Vec<Vec<_>> = productions.to_vec();
        for production in productions {
          let first_symbol = &production[0];
          if self.token_list.contains(&first_symbol.to_string()) {
            result.insert(first_symbol.clone());
          } else {
            let mut first_set = self.first(first_symbol);
            let mut i = 1;
            let empty_symbol = EMPTY_SYMBOL.get_value().to_string();
            while i < production.len() && first_set.contains(&empty_symbol) {
              first_set.remove(&empty_symbol);
              result.extend(first_set);
              let next_symbol = &production[i];
              first_set = self.first(next_symbol);
              i += 1;
            }
            result.extend(first_set);
          }
        }
      }
    }
    // match symbol {
    //   symbol if self.token_list.contains(&symbol.to_string()) => {
    //     result.insert(symbol.to_string());
    //   }
    //   symbol if !self.token_list.contains(&symbol.to_string()) => {
    //     if let Some(productions) = self.pro_list.get(symbol) {
    //       let productions: Vec<Vec<_>> = productions.to_vec();
    //       for production in productions {
    //         let first_symbol = &production[0];
    //         match first_symbol {
    //           TokenType::Terminal(_) => {
    //             result.insert(first_symbol.clone());
    //           }
    //           TokenType::NotTerminal => {
    //             let mut first_set = self.first(first_symbol);
    //             let mut i = 1;
    //             let empty_symbol = EMPTY_SYMBOL.clone();
    //             while i < production.len() && first_set.contains(&empty_symbol) {
    //               first_set.remove(&empty_symbol);
    //               result.extend(first_set);
    //               let next_symbol = &production[i];
    //               first_set = self.first(next_symbol);
    //               i += 1;
    //             }
    //             result.extend(first_set);
    //           }
    //         }
    //       }
    //     }
    //   }
    // }

    self.first_sets.insert(symbol.to_string(), result.clone());
    result
  }

  fn calculate_first_sets(&mut self) {
    let non_terminals: Vec<_> = self.pro_list.keys().cloned().collect();

    for non_terminal in non_terminals {
      self.first(&non_terminal);
    }
  }

  pub(crate) fn first_symbols(&self, symbols: &[String], fallback: &String) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut epsilon = true;

    for symbol in symbols {
      epsilon = false;

      if self.token_list.contains(symbol) {
        result.insert(symbol.clone());
        break;
      }else {
        let first_set = self.first_sets.get(symbol).unwrap();
        let empty_symbol = EMPTY_SYMBOL.get_value().to_string();
        if first_set.contains(&empty_symbol) {
          epsilon = true;
          result.extend(first_set.clone().into_iter().filter(|x| *x != empty_symbol));
        } else {
          result.extend(first_set.clone().into_iter());
          break;
        }
      }
      // match symbol {
      //   TokenType::Terminal(_) => {
      //     result.insert(symbol.clone());
      //     break;
      //   }
      //   TokenType::NotTerminal => {
      //     let first_set = self.first_sets.get(symbol).unwrap();
      //     let empty_symbol = EMPTY_SYMBOL.clone();
      //     if first_set.contains(&empty_symbol) {
      //       epsilon = true;
      //       result.extend(first_set.clone().into_iter().filter(|x| *x != empty_symbol));
      //     } else {
      //       result.extend(first_set.clone().into_iter());
      //       break;
      //     }
      //   }
      // }
    }

    if epsilon {
      result.insert(fallback.clone());
    }

    result
  }
}