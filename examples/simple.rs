use std::io::stdout;
use simple_text_template::context::{Builder, ContextBuilder, ObjectBuilder};
use simple_text_template::render_to_writer;

fn main() {
  let context = ContextBuilder::new()
    .add_value("value", "Value")
    .add_value("true", true)
    .add_value("false", true)
    .add_values("nums", ["2", "3", "4"])
    .add_values("list", ["First", "Second", "Third"])
    .build();

  let text = r#"
$value
$if true: This is true
$if false: You wont see this
$if true:
This is true
$end
$if false:
You wont see this
$end
$for val in list:
$val
$end
1 $for n in nums: $n $end 5 6
1 $for n in nums: $n
"#;

  render_to_writer(&context, text, stdout()).unwrap();
}
