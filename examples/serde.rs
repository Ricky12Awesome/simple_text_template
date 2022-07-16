use serde::Serialize;
use simple_text_template::context::serde::ToContext;
use simple_text_template::render_to_string;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
struct Context<'a> {
  item: &'a str,
  items: Vec<&'a str>,
  object: HashMap<&'a str, &'a str>,
}

impl ToContext for Context<'static> {}

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
  let context = Context::default().to_context().unwrap();
  let text = "";
  let rendered = render_to_string(&context, text).unwrap();

  println!("{rendered}");
}
