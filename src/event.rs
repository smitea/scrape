use std::iter::Map;

use crate::Value;

pub type Event = Map<String, Value>;

pub trait ToEvent: Send{
    fn to(&self) -> Event;
}