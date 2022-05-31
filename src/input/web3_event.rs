use super::Input;
use crate::{Config, Result, decode::Decoder, new_runtime, process::Processor};
use tokio::sync::mpsc::Sender;
use web3::{transports::Http, Web3};

pub struct Web3EventInput {
    web3: Web3<Http>,
}

impl Web3EventInput {
    fn new<C: Config>(config: &C) -> Result<Self> {
        let rpc_uri: String = config.get_value("")?;

        let http = web3::transports::Http::new(&rpc_uri)?;
        let web3 = web3::Web3::new(http);
        return Ok(Web3EventInput { web3 });
    }
}

impl Input for Web3EventInput {
    fn start<C: Config, P: Processor, D: Decoder>(
        config: &C,
        processor: P,
        decoder: D,
        sender: Sender<D>,
    ) -> Result<()> {
        let input = Self::new(config);
        new_runtime(config, "input.max_thread", async {})?;

        return Ok(());
    }
}
