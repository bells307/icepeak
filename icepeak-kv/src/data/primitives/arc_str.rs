use crate::{data::Data, DataBytes};
use std::{str::Utf8Error, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArcString(Arc<str>);

impl ArcString {
    pub fn new(s: Arc<str>) -> Self {
        Self(s)
    }
}

impl From<&str> for ArcString {
    fn from(s: &str) -> Self {
        Self::new(s.into())
    }
}

impl Data for ArcString {
    type Error = Utf8Error;

    fn into_data(self) -> DataBytes {
        self.0.into()
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data.as_slice())?;
        Ok(Self::from(s))
    }
}
