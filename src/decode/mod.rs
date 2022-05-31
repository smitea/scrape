use crate::{Result, ToEvent};

mod transfer;

pub trait Decoder {
    fn decode<T: ToEvent>(bytes: [u8; 0]) -> Result<T>;
}
