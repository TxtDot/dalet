use dalet::daletl::{Argument, Body, Tag, Tid};

fn main() {
    let _ = Tag::new(
        Tid::H,
        Body::Text("I am Heading".to_string()),
        Argument::Null,
    );
    println!("Hello, world!");
}
