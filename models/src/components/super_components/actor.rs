use crate::components::base_components::{
    actor::{
        RawActor, actor_type::ActorType, ethnicity::Ethnicity, known_group::KnownGroup,
        religion::Religion,
    },
    location::country::CountryZone,
};

use super::ActorName;

pub struct Actor {
    pub name: ActorName,
    pub country: CountryZone,
    pub known_group: KnownGroup,
    pub ethnicity: Ethnicity,
    pub religion: (Religion, Religion),
    pub actor_type: (ActorType, ActorType, ActorType),
}

impl<'a> From<RawActor<'a>> for Actor {
    fn from(value: RawActor) -> Self {
        Self {
            name: ActorName::from(value.name),
            country: CountryZone::from(value.country_code),
            known_group: KnownGroup::from(value.known_group_code),
            ethnicity: Ethnicity::from(value.ethnic_code),
            religion: (
                Religion::from(value.religion_1_code),
                Religion::from(value.religion_2_code),
            ),
            actor_type: (
                ActorType::from(value.type_1_code),
                ActorType::from(value.type_2_code),
                ActorType::from(value.type_3_code),
            ),
        }
    }
}
