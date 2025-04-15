use crate::components::base_components::actor::{
    ActorCountry, ActorName, actor_type::ActorType, ethnicity::Ethnicity, known_group::KnownGroup,
    religion::Religion,
};

pub struct Actor {
    pub name: ActorName,
    pub country: ActorCountry,
    pub known_group: KnownGroup,
    pub ethnicity: Ethnicity,
    pub religion: Religion,
    pub actor_type: ActorType,
}
