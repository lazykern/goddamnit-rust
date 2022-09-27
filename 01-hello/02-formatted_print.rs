fn main() {

  println!("Formatting just {{}}: {} {}", 1, 2);

  println!("Formatting with positional arguments: {2}{2}{0}{1}", 'a', 'b', 'c');

  println!("Formatting with named arguments: {a}{b}{c}", a = 1, b = 2, c = 3);

  println!("\nFormatting numbers");
  let number = 69420;
  println!("Decimal: {}", number);
  println!("Binary: {:b}", number);
  println!("Octal: {:o}", number);
  println!("Hex (x): {:x}", number);
  println!("Hex (X): {:x}", number);

  let align_width = 10;

  println!("\nAlignment ({width})", width = align_width);
  println!("Left aligned:\n{:<align_width$}", "left");

  println!("Right aligned:\n{:>align_width$}", "right");

  println!("Right aligned with zero filled:\n{:0>align_width$}", "right");

  println!("Center aligned:\n{:^align_width$}", "center");

  let f_number: f64 = 1.0;
  let width: usize = 10;
  println!("\nCapture format argument from variable {f_number:>width$}");

  #[derive(Debug)]
  struct MyInt32(i32);
  println!("\nDebug: {:?}", MyInt32(3));
}
