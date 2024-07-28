use serde::Serialize;

#[derive(Serialize, Debug)]
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

#[derive(Debug)]
pub enum Body {
    Text(String),
    Tag(Box<Tag>),
    Null,
}

impl Serialize for Body {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Body::Text(ref text) => serializer.serialize_str(text),
            Body::Tag(ref tag) => tag.serialize(serializer),
            Body::Null => serializer.serialize_str("null"),
        }
    }
}

#[derive(Debug)]
pub enum Argument {
    Text(String),
    Number(i8),
    Null,
}

impl Serialize for Argument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Argument::Text(ref text) => serializer.serialize_str(text),
            Argument::Number(number) => serializer.serialize_i8(number),
            Argument::Null => serializer.serialize_str("null"),
        }
    }
}
