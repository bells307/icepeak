pub(crate) mod arc_str;

use super::DataTrait;
use crate::Data;
use std::{convert::Infallible, sync::Arc};

// impl DataTrait for String {
//     type Error = FromUtf8Error;
//
//     fn into_data(self) -> Data {
//         Data::new(self.as_bytes().into())
//     }
//
//     fn from_data(data: Data) -> Result<Self, Self::Error> {
//         String::from_utf8(data.into())
//     }
// }
//
// impl DataTrait for Vec<u8> {
//     type Error = Infallible;
//
//     fn into_data(self) -> Data {
//         Data::new(self)
//     }
//
//     fn from_data(data: Data) -> Result<Self, Self::Error> {
//         Ok(data.into_vec())
//     }
// }

impl DataTrait for i8 {
    type Error = Infallible;

    fn into_data(self) -> Data {
        let bytes = self.to_be_bytes();
        Data::new(Arc::new(bytes))
    }

    fn from_data(data: Data) -> Result<Self, Self::Error> {
        Ok(i8::from_be_bytes([data.0[0]]))
    }
}

//
// impl IntoData for i16 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for i32 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for i64 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for i128 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for isize {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for u8 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for u16 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for u32 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for u64 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for u128 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for usize {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for f32 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for f64 {
//     fn into_data(self) -> Data {
//         let bytes = self.to_be_bytes();
//         Data::new(Arc::new(bytes))
//     }
// }
//
// impl IntoData for bool {
//     fn into_data(self) -> Data {
//         let bytes = [self as u8];
//         Data::new(Arc::new(bytes))
//     }
// }
