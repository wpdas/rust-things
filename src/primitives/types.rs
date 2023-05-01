fn main() {
  // Variables can be type annotated.
  let logical: bool = true;
  
  let a_float: f64 = 1.0; // Regular annotation -> 1
  let an_integer = 5i32; // Suffix annotation -> 5
  
  // Or a default will be used.
  let default_float = 3.0; // deafults type as f64
  let default_integer = 7; // defaults type as i32
  
  // A type can also be inferred from context.
  let mut inferred_type = 12; // Type i64 is inferred from another line.
  inferred_type = 4294967296i64; // force to be i64 (used for floating numbers)
  
  // A mutable variable's value can be changed. IMPORTANT
  let mut mutable = 12; // Mutable `i32`
  mutable = 21;
  
  // NOTE: Error! The type of a variable can't be changed.
  // mutable = true;
  
  // NOTE: This line below is going to break because can no be changed (is not mutable)
  // a_float = 1.4;
  
  // Variables can be overwritten with shadowing. (now it's not mutable)
  let mutable = true;
  // mutable = 12; // WILL FAIL
  
  println!("{0}", mutable)
}