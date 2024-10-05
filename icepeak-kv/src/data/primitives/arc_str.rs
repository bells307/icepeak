use std::{str::Utf8Error, sync::Arc};

use crate::{data::DataTrait, Data};

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

impl DataTrait for ArcString {
    type Error = Utf8Error;

    fn into_data(self) -> Data {
        self.0.into()
    }

    fn from_data(data: Data) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data.as_slice())?;
        Ok(Self::from(s))
    }
}
