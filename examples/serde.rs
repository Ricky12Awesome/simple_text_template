use serde::Serialize;
use simple_text_template::context::serde::to_context;

#[derive(Serialize, Debug, Clone)]
struct Context<'a> {
  item: &'a str,
  items: Vec<&'a str>,
}

impl Default for Context<'static> {
  fn default() -> Self {
    Self {
      item: "Stuff",
      items: vec!["A", "B", "C"],
    }
  }
}

fn main() {
  let context = to_context(&Context::default()).unwrap();

  println!("{context:#?}");
}