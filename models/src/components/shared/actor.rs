use crate::components::cameo::actors::{
    ActorCode, primary_roles::PrimaryRole, religious_codes::Religion,
};

pub struct Actor {
    pub code: Option<ActorCode>,
    pub name: Option<String>,
    pub known_group_code: Option<ActorCode>,
    pub religion_code: (Option<Religion>, Option<Religion>),
    pub roles: (
        Option<PrimaryRole>,
        Option<PrimaryRole>,
        Option<PrimaryRole>,
    ),
}
