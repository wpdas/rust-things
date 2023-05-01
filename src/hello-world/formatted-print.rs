fn main() {
  println!("{} days", 31);

  // Positional arguments can be used;
  // 0 = Alice
  // 1 = Bob
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // As can named arguments.
  println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

  // Different formatting can be invoked by specifying the format character after a `:`
  println!("Base 10: {}", 69420); // decimal (normal) - 69420
  println!("Base 2: (binary): {:b}", 69420); // 10000111100101100
  println!("Base 8 (octal):        {:o}", 69420); // 207454
  println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
  println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

  // You can use named arguments in the format specifier by appending a `$`.
  println!("{number:0>width$}", number=1, width=5);

  // Rust even checks to make sure the correct number of arguments are used.
  println!("My name is {0}, {1} {0}", "Bond", "Jame");
  // FIXME ^ Add the missing argument: "James"

  #[allow(dead_code)] // disable `dead_code` which warn against unused module
  struct Structure(i32);
  // This will not compile because `Structure` does not implement
  // fmt::Display.
  // println!("This struct `{}` won't print...", Structure(3));
  // TODO ^ Try uncommenting this line

  let pi = 3.1411592;
  println!("Pi is roughly {pi:.*}", 3); // Make it to have 3 fractional digits
  // check out more here: https://doc.rust-lang.org/std/fmt/
}