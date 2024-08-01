use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    El(NotNullBody),
    H(String, HeadingArgument),
    P(NotNullBody),
    Br,
    Ul(Vec<Tag>),
    Ol(Vec<Tag>),
    Row(Vec<Tag>, AlignArgument),
    Link(Body, String),
    Navlink(Body, String),
    Btn(Body, String),
    NavBtn(Body, String),
    Img(String),
    Table(Vec<Tag>),
    Tcol(Vec<Tag>),
    Tpcol(Vec<Tag>),
    Hr,
    B(String),
    I(String),
    Bq(NotNullBody),
    Footlnk(StringOrNumberArgument),
    Footn(String, StringOrNumberArgument),
    A(StringOrNumberArgument),
    S(String),
    Sup(String),
    Sub(String),
    Disc(NotNullBody),
    Bl(NotNullBody, AlignArgument),
    Carousel(Vec<Tag>),
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum HeadingArgument {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum AlignArgument {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringOrNumberArgument {
    String(String),
    Number(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Argument {
    Text(String),
    Number(i8),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotNullArgument {
    Text(String),
    Number(i8),
}

impl NotNullArgument {
    pub fn to_argument(self) -> Argument {
        match self {
            NotNullArgument::Number(v) => Argument::Number(v),
            NotNullArgument::Text(v) => Argument::Text(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotNullBody {
    Text(String),
    Tags(Vec<Tag>),
}

impl NotNullBody {
    pub fn to_body(self) -> Body {
        match self {
            NotNullBody::Text(v) => Body::Text(v),
            NotNullBody::Tags(v) => Body::Tags(v),
        }
    }
}
