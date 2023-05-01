#[allow(dead_code)]
fn private_function() {
  println!("called `my_mod::private_function()`");
}

pub fn function() {
  println!("called `my_mod::function()`");
}

pub mod nested {
  pub fn function() {
    println!("called `my_mod::nested::function()`");
  }
}