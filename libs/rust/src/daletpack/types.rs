use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaletPackError {
    StrMaxSizeExceeded,
    ArrMaxSizeExceeded,
    RootMaxSizeExceeded,
    ZstdCompressError,
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive, Copy)]
#[repr(u8)]
pub enum TypeId {
    Int8 = 1,
    Str8 = 4,
    Str16,
    Str32,
    TagArray,
    TagArrayEnd,
    TagId,
    TagIdBody,
    TagIdArgument,
    TagIdBodyArgument,
}
