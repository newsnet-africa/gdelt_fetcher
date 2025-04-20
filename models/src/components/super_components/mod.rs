use super::base_components::raw_types::RawActorName;

pub mod actor;
pub mod event_action;
pub mod geography;

pub struct ActorName(String);

impl<'a> From<RawActorName<'a>> for ActorName {
    fn from(value: RawActorName) -> Self {
        ActorName(value.0.to_string())
    }
}
