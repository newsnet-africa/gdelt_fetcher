use crate::types::event_table::actor::ActorName;

#[derive(Debug)]
pub enum ActorType {
    Person(ActorName),
    Organisation(ActorName),
    Other(ActorName),
}
