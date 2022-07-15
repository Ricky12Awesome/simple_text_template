use crate::context::Context;
use crate::renderer::{Error, Renderer};

pub mod context;
pub mod renderer;

pub fn render<S>(context: &Context, text: S) -> Result<String, Error>
where
  S: AsRef<str>,
{
  Renderer::new(context).render(text)
}
