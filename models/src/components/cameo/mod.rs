use actors::ActorCode;
use verb::EventCode;

pub mod actors;
pub mod verb;

#[repr(transparent)]
pub struct CAMEOCodeRaw(String);

pub enum CAMEOCode {
    Actor(ActorCode),
    Event(EventCode),
}
