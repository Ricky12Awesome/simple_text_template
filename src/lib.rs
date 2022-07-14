use context::Context;

pub mod context;

pub struct Renderer {
  _context: Context,
}

impl Renderer {
  pub fn new(context: Context) -> Self {
    Self { _context: context }
  }

  pub fn render<S>(&self, text: S) -> String
  where
    S: ToString,
  {
    text.to_string()
  }
}
