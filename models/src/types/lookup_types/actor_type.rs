use crate::types::event_table::actor::ActorName;

pub enum ActorType {
    Person(ActorName),
    Organisation(ActorName),
    Other(ActorName),
}
