use std::io::stdout;

use simple_text_template::context::ContextBuilder;
use simple_text_template::render_to_writer;

fn main() {
  #[rustfmt::skip]
  let context = ContextBuilder::new()
    .set_value("value", "Value")
    .set_value("true", true)
    .set_value("false", false)
    .set_list("nums", ["2", "3", "4"])
    .set_list("list", ["First", "Second", "Third"])
    .set_value("nested", vec![
      ContextBuilder::new().set_value("a", vec![
        ContextBuilder::new().set_value("b", vec![
          ContextBuilder::new().set_value("c", "Value")
            .build_to_value(); 3])
          .build_to_value(); 3])
        .build_to_value(); 3])
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

$for nested in nested: $for a in nested.a: $for b in a: $for c in b: $c

$for val in list:
$val
$end
1 $for n in nums: $n  $end 5 6
1 $for n in nums: $n
"#;

  render_to_writer(context, text, stdout()).unwrap();
}
