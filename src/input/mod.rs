pub mod web3_event;
pub mod web3_rpc;

use tokio::sync::mpsc::Sender;
use crate::{decode::Decoder, process::Processor, Config, Result};

pub trait Input {
    fn start<C: Config, P: Processor, D: Decoder>(
        config: &C,
        processor: P,
        decoder: D,
        sender: Sender<D>,
    ) -> Result<()>;
}
