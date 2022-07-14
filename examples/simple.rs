use simple_text_template::context::{Builder, ContextBuilder, ObjectBuilder};
use simple_text_template::Renderer;

fn main() {
  let context = ContextBuilder::new()
    .add_value("item", "Stuff")
    .add_values("items", ["A", "B", "C"])
    .add_value("object", ObjectBuilder::new().add_value("value", "string")) //
    .build();

  println!("item: {:?}", context.get_string("item"));
  println!("items: {:?}", context.get_string("items"));
  println!("items: {:?}", context.get_list("items"));
  println!("object: {:?}", context.get_object("object"));

  let renderer = Renderer::new(context);
  let text = r#"$item
$object.value
$for item in items: $item
"#;

  let rendered = renderer.render(text);

  println!("{rendered}");
}
