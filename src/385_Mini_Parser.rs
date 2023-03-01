// 385. Mini Parser
// https://leetcode.cn/problems/mini-parser/
// 4 ms 50%
// 3 MB 0%


// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }

impl Solution {
  pub fn deserialize(s: String) -> NestedInteger {
    if !s.starts_with('[') {
      let num = s.parse::<i32>().unwrap();
      return NestedInteger::Int(num);
    }
  
    let mut idx = 1;
    let mut items: Vec<&str> = Vec::new();
    let data: Vec<char> = s.chars().collect();
    while idx < data.len()-1 {
      // is a single integer
      if data[idx] == ',' {
        idx += 1;
        continue;
      }
  
      let mut offset = 0;
      // is a nested list
      if data[idx] == '[' {
        let mut left_count = 1;
        while idx + offset < data.len()-1 && left_count > 0 {
          offset += 1;
          if data[idx + offset] == '[' {
            left_count += 1;
          }
          if data[idx + offset] == ']' {
            left_count -= 1;
          } 
        }
        items.push(&s[idx..idx+offset+1]);
      }
      // is a integer
      else {
          while idx + offset < data.len()-1 && data[idx + offset] != ',' {
            offset += 1;
          }
          items.push(&s[idx..idx+offset]);
      }
  
      idx += offset;
      idx += 1;
    }
  
    let mut result = vec![];
  
    for item in items {
      result.push(Solution::deserialize(String::from(item)));
    }
    
    NestedInteger::List(result)
  }
}


/// 有限状态机词法分析，递归向下语法分析 (Can't understand anymore)
/* grammar.bnf

lang ::= expr
expr ::= int | list
list ::= l_bracket list_items r_bracket
list_items ::= non_empty_list_items?
non_empty_list_items ::= expr(comma expr)*
int ::= (neg_sign)?nat
nat ::= non_zero_digit(digit)*
non_zero_digit ::= '1' | '2' | ... | '9'
digit ::= '0' | '1' | ... | '9' 
neg_sign ::= '-'
comma ::= ','
l_bracket ::= '['
r_bracket ::= ']'
*/


impl Solution {
  pub fn deserialize(s: String) -> NestedInteger {
    parse(scan(s.as_bytes()))
  }
}

#[derive(PartialEq, Eq)]
enum Token {
  Comma,
  LBracket,
  RBracket,
  NegSign,
  Nat(i32),
}

fn parse(tokens: Vec<Token>) -> NestedInteger {
  let mut parser = Parser::new(tokens);
  parser.parse()
}

struct Parser {
  tokens: Vec<Token>,
  i: usize,
}

impl Parser {
  fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      i: 0,
    }
  }

  fn parse(&mut self) -> NestedInteger {
    self.lang()
  }
  fn lang(&mut self) -> NestedInteger {
    self.expr().unwrap()
  }

  fn expr(&mut self) -> Option<NestedInteger> {
    self.int().or_else(|| self.list())
  }
  fn int(&mut self) -> Option<NestedInteger> {
    let start_at = self.i;
    let neg_sign = self.neg_sign();
    let nat = self.nat();

    if let Some(nat) = nat {
      let int = if neg_sign {
        NestedInteger::Int(-nat)
      } else {
        NestedInteger::Int(nat)
      };
      Some(int)
    } else {
      self.i = start_at;
      None
    }
  }
  fn list(&mut self) -> Option<NestedInteger> {
    self.l_bracket()?;
    let nis = self.list_items();
    self.r_bracket().unwrap();
    Some(NestedInteger::List(nis))
  }
  fn neg_sign(&mut self) -> bool {
    self.eat(Token::NegSign).map_or(false, |_| true)
  }
  fn nat(&mut self) -> Option<i32> {
    if let Token::Nat(n) = self.tokens[self.i] {
      self.i += 1;
      Some(n)
    } else {
      None
    }
  }
  fn l_bracket(&mut self) -> Option<()> {
    self.eat(Token::LBracket)
  }
  fn r_bracket(&mut self) -> Option<()> {
    self.eat(Token::RBracket)
  }
  fn list_items(&mut self) -> Vec<NestedInteger> {
    self.non_empty_list_items().unwrap_or_else(|| vec![])
  }
  fn non_empty_list_items(&mut self) -> Option<Vec<NestedInteger>> {
    let first_expr = self.expr()?;
    let mut nis = vec![];
    nis.push(first_expr);
    while self.comma().is_some() {
      let expr = self.expr().unwrap();
      nis.push(expr);
    }
    Some(nis)
  }
  fn comma(&mut self) -> Option<()> {
    self.eat(Token::Comma)
  }
  fn eat(&mut self, token: Token) -> Option<()> {
    if token == self.tokens[self.i] {
      self.i += 1;
      Some(())
    } else {
      None
    }
  }
}

fn scan(s: &[u8]) -> Vec<Token> {
  let mut tokens = vec![];
  let mut i = 0usize;
  while i < s.len() {
    let c = s[i];
    match c {
      b'-' => {
        tokens.push(Token::NegSign);
        i += 1;
      }
      b'[' => {
        tokens.push(Token::LBracket);
        i += 1;
      }
      b']' => {
        tokens.push(Token::RBracket);
        i += 1;
      }
      b',' => {
        tokens.push(Token::Comma);
        i += 1;
      }
      c if c.is_ascii_digit() => {
        let mut ds = vec![c];
        loop {
          i += 1;
          if i >= s.len() {
            break;
          }
          if !s[i].is_ascii_digit() {
            break;
          }
          ds.push(s[i]);
        }
        tokens.push(Token::Nat(String::from_utf8(ds).unwrap().parse().unwrap()))
      }
      _ => {
        panic!("impossible")
      }
    }
  }
  tokens
}