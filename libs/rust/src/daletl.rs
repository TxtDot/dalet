use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use num_enum::TryFromPrimitive;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    id: Tid,
    body: Body,
    argument: Argument,
}

impl Tag {
    #[inline]
    pub fn new(id: Tid, body: Body, argument: Argument) -> Tag {
        Tag { id, body, argument }
    }
}

pub fn t_new(id: Tid, body: Body, argument: Argument) -> Tag {
    Tag::new(id, body, argument)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Argument {
    Text(String),
    Number(u8),
    Null,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
/// Tag Id
pub enum Tid {
    El,
    H,
    P,
    Br,
    Ul,
    Ol,
    Row,
    Link,
    Navlink,
    Btn,
    Navbtn,
    Img,
    Table,
    Tcol,
    Tpcol,
    Hr,
    B,
    I,
    Bq,
    Footlnk,
    Footn,
    A,
    S,
    Sup,
    Sub,
    Disc,
    Bl,
    Carousel,
}
