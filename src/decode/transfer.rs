use crate::{Result, ToEvent};

use super::Decoder;

pub struct TransferDecoder {}

impl Decoder for TransferDecoder {
    fn decode<T: ToEvent>(bytes: [u8; 0]) -> Result<T> {
        todo!()
    }
}
