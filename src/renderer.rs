use crate::context::Context;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{}")]
pub enum Error {
  #[error("variable not found: {0}")]
  VariableNotFound(String),
}

pub struct Renderer<'a> {
  context: &'a Context,
  source: &'a str,
}

impl<'a> Renderer<'a> {
  pub fn new(context: &'a Context, source: &'a str) -> Self {
    Self { context, source }
  }

  fn render_value(&mut self) {

  }

  pub fn render(mut self) -> Result<String, Error> {
    let src = self.source;

    let mut buf = String::with_capacity(src.len() * 2);
    let mut statement_start = None;

    for (index, ch) in src.char_indices() {
      if ch == '$' {
        statement_start = Some(index)
      }

      match statement_start {
        Some(start_index) => {
          if ch.is_whitespace() {
            statement_start = None;

            let name = &src[start_index..index][1..];

            match name {
              "if" => {}
              "for" => {}
              name => {
                let value = self.context.get_string(name);

                if let Some(value) = value {
                  buf.push_str(value);
                }
              }
            }
          }
        }
        None => buf.push(ch),
      }
    }

    Ok(buf)
  }
}
