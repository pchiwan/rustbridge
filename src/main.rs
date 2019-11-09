// extern crate markdown;
use markdown::to_html;

macro_rules! greet {
  () => {
    println!("Hello stranger!");
  };
  ($name: expr) => {
    println!("Hello {}!", $name);
  }
}

fn main () {
  // let html : String = markdown::to_html("__This is bold__");
  let html : String = to_html("__This is bold__");

  println!("{}", html);
  greet!();
  greet!(String::from("Shepard"));
}
