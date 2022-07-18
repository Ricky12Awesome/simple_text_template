use thiserror::Error;

use crate::context::{Context, GetValue, SetValue};

#[derive(Error, Debug)]
#[error("{}")]
pub enum Error {
  #[error("variable not found: {0}")]
  VariableNotFound(String),
  #[error("{0}")]
  Io(#[from] std::io::Error),
  #[error("{0}")]
  FromUtf8(#[from] std::string::FromUtf8Error),
}

pub struct Renderer<'a, W> {
  context: Context,
  source: &'a str,
  writer: W,
}

impl<'a, W> Renderer<'a, W>
where
  W: std::io::Write,
{
  pub fn new(context: Context, source: &'a str, writer: W) -> Self {
    Self {
      context,
      source,
      writer,
    }
  }

  fn _render(&mut self, context: Context, text: &'a str) -> Result<(), Error> {
    for token in TokenIter::new(text) {
      match token {
        Item::Normal(text) => write!(self.writer, "{text}")?,
        Item::Var(path) => {
          let value = context
            .get_string(path)
            .ok_or_else(|| Error::VariableNotFound(path.to_string()))?;

          write!(self.writer, "{value}")?;
        }
        Item::If(path, not, true_block, false_block) => {
          let condition = context.get_bool(path);

          if condition ^ not {
            self._render(context.clone(), true_block)?;
          } else if let Some(false_block) = false_block {
            self._render(context.clone(), false_block)?;
          }
        }
        Item::For(id, path, block) => {
          let list = self
            .context
            .get_list(path)
            .ok_or_else(|| Error::VariableNotFound(path.to_string()))?;

          for element in list.clone() {
            let mut context = context.clone();
            context.set_value(id, element);
            self._render(context, block)?;
          }
        }
      }
    }

    Ok(())
  }

  pub fn render(&mut self) -> Result<(), Error> {
    self._render(self.context.clone(), self.source)
  }
}

struct TokenIter<'a> {
  source: &'a str,
  is_token: bool,
}

impl<'a> TokenIter<'a> {
  fn new(source: &'a str) -> Self {
    Self {
      source,
      is_token: false,
    }
  }
}

#[derive(Debug)]
enum Token<'a> {
  /// Any other text
  Normal(&'a str),
  /// ${0}
  Var(&'a str),
  /// $if {1}{0}: ${2} $else: ${3}
  If(&'a str, bool, &'a str, Option<&'a str>),
  /// $for {0} in {1}: {2}
  For(&'a str, &'a str, &'a str),
}

type Item<'a> = Token<'a>;

impl<'a> TokenIter<'a> {
  fn token_start(&mut self) -> Option<Item<'a>> {
    let token_start = self.source.find('$');

    match token_start {
      None => {
        let source = self.source;
        self.source = "";
        Some(Token::Normal(source))
      }
      Some(start) => {
        let source = &self.source[..start];
        self.is_token = true;
        self.source = self.source.get(start + 1..)?;
        Some(Token::Normal(source))
      }
    }
  }

  fn handle_token(&mut self) -> Option<Item<'a>> {
    self.is_token = false;

    let end = self.source.find([' ', '\n']);

    match end {
      None => {
        let source = self.source;
        self.source = "";
        Some(Token::Var(source))
      }
      Some(end) => match &self.source[..end] {
        "if" => self.handle_if_token(),
        "for" => self.handle_for_token(),
        name => {
          self.source = &self.source[end..];
          Some(Token::Var(name))
        }
      },
    }
  }

  #[inline]
  fn hande_block(&mut self, f: impl FnOnce(&'a str) -> Item<'a>) -> Option<Item<'a>> {
    let line_end = self.source.find('\n').unwrap();

    let is_one_liner = self.source[..line_end].len() > 2;

    if is_one_liner {
      let block = &self.source[..line_end];

      match block.find("$end") {
        None => {
          self.source = &self.source[line_end..];
          Some(f(&block[2..]))
        }
        Some(end) => {
          let block = &block[..end - 1];
          self.source = &self.source[end + 5..];
          Some(f(&block[2..]))
        }
      }
    } else {
      self.source = &self.source[line_end..];
      let block_end = self.source.find("$end").unwrap();
      let block = &self.source[..block_end];

      self.source = &self.source[block_end + 5..];

      Some(f(&block[1..]))
    }
  }

  fn handle_if_token(&mut self) -> Option<Item<'a>> {
    let condition_end = self.source.find(':').unwrap();
    let condition = &self.source[3..condition_end];

    self.source = &self.source[condition_end..];

    self.hande_block(|block| Token::If(condition, false, block, None))
  }

  fn handle_for_token(&mut self) -> Option<Item<'a>> {
    let statement_end = self.source.find(':').unwrap();
    let statement = &self.source[4..statement_end];
    let (element, elements) = statement.split_once(" in ").unwrap();

    self.source = &self.source[statement_end..];

    self.hande_block(|block| Token::For(element, elements, block))
  }
}

impl<'a> Iterator for TokenIter<'a> {
  type Item = Item<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.source.is_empty() {
      return None;
    }

    if !self.is_token {
      self.token_start()
    } else {
      self.handle_token()
    }
  }
}
