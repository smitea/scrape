use std::future::Future;

use input::Input;
use input::web3_event::Web3EventInput;
use output::Output;

mod config;
mod datatype;
mod decode;
mod error;
mod event;
mod input;
mod output;
mod value;
mod process;

pub use config::Config;
pub use datatype::DataType;
pub use datatype::*;
pub use error::Error;
pub use error::Result;
pub use event::ToEvent;
use output::console::ConsoleOutput;
use tokio::runtime::Builder;
use tokio::task::JoinHandle;
pub use value::Value;
pub fn run<C: Config>(config: &C) -> Result<()> {
    let buffer_size: i32 = config.get_value("buffer_size")?;
    let (mut sender, reciver) = tokio::sync::mpsc::channel(buffer_size as usize);

    // Web3EventInput::start(config, processor, decoder, sender)?;
    ConsoleOutput::start(config, reciver)?;
    
    Ok(())
}

fn new_runtime<C: Config, K: Into<String>, F>(
    config: &C,
    key: K,
    future: F,
) -> Result<JoinHandle<F::Output>>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let max_thread: i32 = config.get_value(key)?;

    let runtime = Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(max_thread as usize)
        .build()?;
    return Ok(runtime.spawn(future));
}
