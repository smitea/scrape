use tokio::{runtime::Builder, sync::mpsc::Receiver};

use crate::{event::Event, new_runtime, Config, Result};

use super::Output;
pub struct ConsoleOutput {}

impl Output for ConsoleOutput {
    fn start<C: Config>(config: &C, reciver: Receiver<Event>) -> Result<()> {
        new_runtime(config, "output.max_thread", async {
            let mut reciver = reciver;
            while let Some(event) = reciver.recv().await {
                println!("{:?}", event);
            }
        })?;
        return Ok(());
    }
}
