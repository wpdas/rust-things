#[derive(Debug)]
// Similar to interface / type (TS)
struct Person<'a> {
  name: &'a str,
  age: u8
}

fn main() {
  let name = "Wendz";
  let age = 32;
  let peter = Person { name, age };

  // Pretty print
  println!("{:#?}", peter);
}