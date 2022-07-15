use serde::Serialize;
use simple_text_template::context::serde::to_context;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
struct Context<'a> {
  item: &'a str,
  items: Vec<&'a str>,
  object: HashMap<&'a str, &'a str>,
}

impl Default for Context<'static> {
  fn default() -> Self {
    Self {
      item: "Stuff",
      items: vec!["A", "B", "C"],
      object: HashMap::from_iter([
        ("Stuff 1", "localhost/stuff/1"),
        ("Stuff 2", "localhost/stuff/2"),
        ("Stuff 3", "localhost/stuff/3"),
      ]),
    }
  }
}

fn main() {
  let context = to_context(&Context::default()).unwrap();

  println!("{context:#?}");
}
