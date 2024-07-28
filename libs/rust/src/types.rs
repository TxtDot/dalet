use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: i8,
    body: Body,
    argument: Argument,
}

impl Tag {
    pub fn new(id: i8, body: Body, argument: Argument) -> Tag {
        Tag { id, body, argument }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Body {
    Text(String),
    Tag(Box<Tag>),
    Null,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Argument {
    Text(String),
    Number(i8),
    Null,
}
