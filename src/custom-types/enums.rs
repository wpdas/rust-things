enum WebEvent {
  // An `enum` variant may either be `unit-like`
  PageLoad,
  PageUnload,
  // like tuples structs,
  KeyPress(char),
  Past(String),
  // or c-like structures,
  Click {x: i64, y: i64},
}

// A function which takes a `WebEvent` enum as an argument and returns nothing.
fn inspect(event: WebEvent) {
  // essa e a forma de se fazer o switch (do JS)
  match event {
      WebEvent::PageLoad => println!("page loaded"),
      WebEvent::PageUnload => println!("page unloaded"),
      WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
      WebEvent::Past(s) => println!("pasted \"{}\".", s),
      WebEvent::Click { x, y } => {
          println!("clicked at x={}, y={}", x, y)
      }
  }
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted = WebEvent::Past("my text".to_owned());
  let click = WebEvent::Click { x: 20, y: 80 };
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);
}