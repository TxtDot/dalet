#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaletPackError {
    StrMaxSizeExceeded,
    ArrMaxSizeExceeded,
    RootMaxSizeExceeded,
}
