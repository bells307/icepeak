pub(crate) mod arc_str;

use super::Data;
use crate::DataBytes;
use std::{mem, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum PrimitiveCastError {
    #[error("not enough bytes in data, expected: {0}")]
    NotEnoughData(usize),
}

impl Data for i8 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| Self::from_be_bytes([slice[0]]))
    }
}

impl Data for i16 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| Self::from_be_bytes([slice[0], slice[1]]))
    }
}

impl Data for i32 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| {
            Self::from_be_bytes([slice[0], slice[1], slice[2], slice[3]])
        })
    }
}

impl Data for i64 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| {
            Self::from_be_bytes([
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
            ])
        })
    }
}

impl Data for i128 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| {
            Self::from_be_bytes([
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
                slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14],
                slice[15],
            ])
        })
    }
}

impl Data for isize {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| {
            Self::from_be_bytes([
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
            ])
        })
    }
}

impl Data for u8 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| Self::from_be_bytes([slice[0]]))
    }
}

impl Data for u16 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| Self::from_be_bytes([slice[0], slice[1]]))
    }
}

impl Data for u32 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| {
            Self::from_be_bytes([slice[0], slice[1], slice[2], slice[3]])
        })
    }
}

impl Data for u64 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| {
            Self::from_be_bytes([
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
            ])
        })
    }
}

impl Data for u128 {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| {
            Self::from_be_bytes([
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
                slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14],
                slice[15],
            ])
        })
    }
}

impl Data for usize {
    type Error = PrimitiveCastError;

    fn into_data(self) -> DataBytes {
        let bytes = self.to_be_bytes();
        DataBytes::new(Arc::new(bytes))
    }

    fn from_data(data: DataBytes) -> Result<Self, Self::Error> {
        integer_cast(data, |slice| {
            Self::from_be_bytes([
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
            ])
        })
    }
}

#[inline(always)]
fn integer_cast<T>(data: DataBytes, f: impl Fn(&[u8]) -> T) -> Result<T, PrimitiveCastError> {
    let slice = data.as_slice();
    let exp = mem::size_of::<T>();

    if slice.len() < exp {
        Err(PrimitiveCastError::NotEnoughData(exp))
    } else {
        Ok(f(slice))
    }
}
