use simple_text_template::context::{Builder, ContextBuilder, ObjectBuilder};
use simple_text_template::render;

fn main() {
  let context = ContextBuilder::new()
    .add_value("item", "Stuff")
    .add_values("items", ["A", "B", "C"])
    .add_value("object", ObjectBuilder::new().add_value("value", "string"))
    .build();

  println!("item: {:?}", context.get_string("item"));
  println!("items: {:?}", context.get_string("items"));
  println!("items: {:?}", context.get_list("items"));
  println!("object: {:?}", context.get_object("object"));
  println!("object: {:?}", context.get_value("object.value"));
  println!("object: {:?}", context.get_value("object.value.none"));

  let text = r#"$item

$object.value

$for item in items: $item

"#;

  let rendered = render(&context, text).unwrap();

  println!("{rendered}");
}
