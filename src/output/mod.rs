pub mod console;
pub mod current_file;

use tokio::sync::mpsc::Receiver;

use crate::{event::Event, Config, Result};

pub trait Output {
    fn start<C: Config>(config: &C, reciver: Receiver<Event>) -> Result<()>;
}
