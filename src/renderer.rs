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
}

impl<'a> Renderer<'a> {
  pub fn new(context: &'a Context) -> Self {
    Self { context }
  }

  pub fn render<S>(&'a self, text: S) -> Result<String, Error>
  where
    S: AsRef<str>,
  {
    Ok(text.as_ref().to_string())
  }
}
