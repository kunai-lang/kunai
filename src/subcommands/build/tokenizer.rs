extern crate regex;
use regex::{Captures, Regex, Replacer};

use std::char;
use std::u32;

pub struct Token {
  pub token: String,
  pub file_name: String,
  pub token_type: TokenType,
  pub line: usize,
  pub index: usize,
}

pub enum TokenType {
  StringLiteral,
  ByteLiteral,
  IntegerLiteral,
  FloatingLiteral,
  Keyword,
  Identifier,
  Operator,
  Punctuator,
}

impl Clone for TokenType {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for TokenType {}

pub struct TokenRegex {
  pub exp: Regex,
  pub token_type: TokenType,
}

pub struct ReplaceUnicodeEscape {}

impl Replacer for ReplaceUnicodeEscape {
  fn replace_append(&mut self, caps: &Captures, dst: &mut String) {
    let m = caps.get(0).unwrap().as_str();
    if m == "\\\\" {
      *dst += m;
    } else {
      let c = char::from_u32(u32::from_str_radix(&m[2..m.len()], 16).expect("")).expect("");
      *dst += &String::from(c);
    }
  }
}

pub fn tokenize(code: String, file_name: &str) -> Vec<Token> {
  let unicode_escape = Regex::new("\\\\\\\\|\\\\u[0-9a-fA-F]{4}").unwrap();
  let new_code = unicode_escape.replace_all(&code, ReplaceUnicodeEscape {});

  let regices = vec![
    TokenRegex {
      exp: Regex::new("^[_a-zA-Z][_a-zA-Z0-9]*$").unwrap(),
      token_type: TokenType::Identifier,
    },
    TokenRegex {
      exp: Regex::new("^\"(?:[^\"\\\\]|\\\\.)*\"$").unwrap(),
      token_type: TokenType::StringLiteral,
    },
    TokenRegex {
      exp: Regex::new("^'(\\\\)?.'$").unwrap(),
      token_type: TokenType::ByteLiteral,
    },
    TokenRegex {
      exp: Regex::new("^([0-9]+(l|L|f|F|d|D)?|0x[0-9a-fA-F]+)$").unwrap(),
      token_type: TokenType::IntegerLiteral,
    },
    TokenRegex {
      exp: Regex::new("^([0-9]+\\.[0-9]+(((e|E)(-|\\+)[0-9]+)|(f|F|d|D)?))$").unwrap(),
      token_type: TokenType::FloatingLiteral,
    },
    TokenRegex {
      exp: Regex::new("^[!-/:-@\\[-`{-~]$").unwrap(),
      token_type: TokenType::Punctuator,
    },
  ];

  let full_exp =
    Regex::new("\"(?:[^\"\\\\]|\\\\.)*\"|'(\\\\)?.'|#.*|[_a-zA-Z][_a-zA-Z0-9]*|([0-9]+(l|L|f|F|d|D)?|0x[0-9a-fA-F]+)|([0-9]+\\.[0-9]+(((e|E)(-|\\+)[0-9]+)|(f|F|d|D)?))|[!-/:-@\\[-`{-~]|\n|.").unwrap();
  let comment_exp = Regex::new("^#.*$").unwrap();
  let white_space_exp = Regex::new("^\\s").unwrap();

  let mut line: usize = 0;
  let mut line_length: usize = 0;

  let mut tokens: Vec<Token> = Vec::new();
  for caps in full_exp.captures_iter(&new_code.as_ref()) {
    let m = caps.get(0).unwrap();
    let substr = &new_code[m.start()..m.end()];
    if substr == "\n" {
      line += 1;
      line_length = new_code[0..m.start()].len();
    } else if !comment_exp.is_match(substr) && !white_space_exp.is_match(substr) {
      let mut index = new_code[0..m.start()].len() - line_length;
      if line == 0 {
        index += 1;
      }

      for exp in &regices {
        if exp.exp.is_match(substr) {
          tokens.push(Token {
            token: substr.to_string(),
            file_name: file_name.to_string(),
            token_type: exp.token_type.clone(),
            line: line,
            index: index,
          });
        }
      }
    }
  }

  return tokens;
}
