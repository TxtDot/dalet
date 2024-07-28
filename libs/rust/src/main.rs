use dalet::{Argument, Body, Tag};

fn main() {
    let _ = Tag::new(1, Body::Text("I am Heading".to_string()), Argument::Null);
    println!("Hello, world!");
}
