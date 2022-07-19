use crate::context::Context;
use crate::renderer::{Error, Renderer};
use std::io::Write;

pub mod context;
pub mod renderer;
pub mod tokenizer;

pub fn render_to_writer<C, S, W>(context: C, source: S, writer: W) -> Result<(), Error>
where
  C: Into<Context>,
  S: AsRef<str>,
  W: Write,
{
  Renderer::new(context.into(), source.as_ref(), writer).render()?;

  Ok(())
}

pub fn render_to_string<C, S>(context: C, source: S) -> Result<String, Error>
where
  C: Into<Context>,
  S: AsRef<str>,
{
  let mut buf = Vec::<u8>::with_capacity(source.as_ref().len() * 2);

  render_to_writer(context, source, &mut buf)?;

  Ok(String::from_utf8(buf)?)
}
