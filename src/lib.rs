use std::borrow::Borrow;
use crate::context::Context;
use crate::renderer::{Error, Renderer};

pub mod context;
pub mod renderer;

pub fn render<C, S>(context: C, source: S) -> Result<String, Error>
where
  C: Borrow<Context>,
  S: AsRef<str>,
{
  Renderer::new(context.borrow(), source.as_ref()).render()
}
