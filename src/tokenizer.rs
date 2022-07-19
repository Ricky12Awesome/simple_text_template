use thiserror::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token<'a> {
  Text(&'a str),
  Variable(&'a str),
  If {
    not: bool,
    variable: &'a str,
    true_block: &'a str,
    false_block: &'a str,
  },
  For {
    name: &'a str,
    variable: &'a str,
    block: &'a str,
  },
  /// only used to tell the iterator to end
  End,
}

#[derive(Error, Debug, PartialEq)]
#[error("{}")]
pub enum Error {
  #[error("no end")]
  NoEnd,
  #[error("Invalid if block")]
  InvalidIfBlock,
}

pub struct TokenizerIter<'a> {
  source: &'a str,
  token: bool,
}

impl<'a> TokenizerIter<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      source,
      token: false,
    }
  }

  fn find_token(&mut self) -> Item<'a> {
    match self.source.find('$') {
      None => {
        let source = self.source;
        self.source = "";
        Ok(Token::Text(source))
      }
      Some(start) => {
        let source = &self.source[..start];
        self.source = &self.source[start + 1..];
        self.token = true;

        if source.is_empty() {
          self.__next()
        } else {
          Ok(Token::Text(source))
        }
      }
    }
  }

  fn token(&mut self) -> Item<'a> {
    let end = self.source.find([' ', '\n', '$']);

    match end {
      None => {
        let name = self.source;
        self.source = "";
        Ok(Token::Variable(name))
      }
      Some(end) => {
        let name = &self.source[..end];
        self.source = &self.source[end..];
        match name {
          "if" => self.if_token(),
          "for" => self.for_token(),
          _ => Ok(Token::Variable(name)),
        }
      }
    }
  }

  fn if_token(&mut self) -> Item<'a> {
    self.source = &self.source[1..];

    let (name, _) = self.source.split_once(':').ok_or(Error::InvalidIfBlock)?;
    self.source = &self.source[name.len() + 1..];
    let name = name.trim();
    let block = self.block();

    Ok(Token::If {
      not: false,
      variable: name,
      true_block: block,
      false_block: ""
    })
  }

  fn block(&mut self) -> &'a str {
    let eol = self.source.find('\n').unwrap_or(self.source.len());
    let line = &self.source[..eol];

    if !line.is_empty() {
      self.source = &self.source[eol..];
      line
    } else {
      ""
    }
  }

  fn for_token(&mut self) -> Item<'a> {
    Ok(Token::End)
  }

  fn __next(&mut self) -> Item<'a> {
    if self.source.is_empty() {
      return Ok(Token::End);
    }

    if !self.token {
      self.find_token()
    } else {
      self.token = false;
      self.token()
    }
  }
}

type Item<'a> = Result<Token<'a>, Error>;

impl<'a> Iterator for TokenizerIter<'a> {
  type Item = Item<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    match Self::__next(self) {
      Ok(Token::End) => None,
      result => Some(result),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::tokenizer::{Token, TokenizerIter};

  mod text_and_variables {
    use super::*;

    #[test]
    fn empty_text() {
      let mut iter = TokenizerIter::new("");

      assert_eq!(None, iter.next());
    }

    #[test]
    fn normal_text() {
      let mut iter = TokenizerIter::new("some text");

      assert_eq!(Some(Ok(Token::Text("some text"))), iter.next());
      assert_eq!(None, iter.next());
    }

    #[test]
    fn variable() {
      let mut iter = TokenizerIter::new("$variable");

      assert_eq!(Some(Ok(Token::Variable("variable"))), iter.next());
      assert_eq!(None, iter.next());
    }

    #[test]
    fn variable_with_inline() {
      let mut iter = TokenizerIter::new("123 $variable 456");

      assert_eq!(Some(Ok(Token::Text("123 "))), iter.next());
      assert_eq!(Some(Ok(Token::Variable("variable"))), iter.next());
      assert_eq!(Some(Ok(Token::Text(" 456"))), iter.next());
      assert_eq!(None, iter.next());
    }

    #[test]
    fn variable_with_newline() {
      let mut iter = TokenizerIter::new("\n$variable\n");

      assert_eq!(Some(Ok(Token::Text("\n"))), iter.next());
      assert_eq!(Some(Ok(Token::Variable("variable"))), iter.next());
      assert_eq!(Some(Ok(Token::Text("\n"))), iter.next());
      assert_eq!(None, iter.next());
    }

    #[test]
    fn variable_multi() {
      let mut iter = TokenizerIter::new("$variable1 $variable2$variable3");

      assert_eq!(Some(Ok(Token::Variable("variable1"))), iter.next());
      assert_eq!(Some(Ok(Token::Text(" "))), iter.next());
      assert_eq!(Some(Ok(Token::Variable("variable2"))), iter.next());
      assert_eq!(Some(Ok(Token::Variable("variable3"))), iter.next());
      assert_eq!(None, iter.next());
    }
  }

  mod ifs {
    use super::*;

    #[test]
    fn if_one_liner() {
      let mut iter = TokenizerIter::new("$if variable: $variable");

      assert_eq!(
        Some(Ok(Token::If {
          not: false,
          variable: "variable",
          true_block: "$variable",
          false_block: "",
        })),
        iter.next()
      );
    }

    #[test]
    fn if_not() {
      let mut iter = TokenizerIter::new("$if !variable: $variable");

      assert_eq!(
        Some(Ok(Token::If {
          not: true,
          variable: "variable",
          true_block: "$variable",
          false_block: "",
        })),
        iter.next()
      );
    }

    #[test]
    fn if_nested() {
      let mut iter = TokenizerIter::new("$if variable.a: $if variable.b: $variable.b");

      assert_eq!(
        Some(Ok(Token::If {
          not: false,
          variable: "variable.a",
          true_block: "$if variable.b: $variable.b",
          false_block: "",
        })),
        iter.next()
      );
    }

    #[test]
    fn if_multiline() {
      let mut iter = TokenizerIter::new("$if variable.a: \n$if variable.b: \n$variable.b\n$end\n$end");

      assert_eq!(
        Some(Ok(Token::If {
          not: false,
          variable: "variable.a",
          true_block: "$if variable.b: \n$variable.b\n",
          false_block: "",
        })),
        iter.next()
      );
    }
  }
}
