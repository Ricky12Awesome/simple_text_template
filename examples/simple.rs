use std::io::stdout;

use simple_text_template::context::{Builder, ContextBuilder};
use simple_text_template::render_to_writer;

fn main() {
  let context = ContextBuilder::new()
    .add_value("value", "Value")
    .add_value("true", true)
    .add_value("false", false)
    .add_values("nums", ["2", "3", "4"])
    .add_values("list", ["First", "Second", "Third"])
    .build();

  let text = r#"
$value

$if true: $value
$if false: You wont see this

$if true:
$value
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
