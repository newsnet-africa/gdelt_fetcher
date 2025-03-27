pub enum Assault {
    NotSpecified,
    AbductHijackOrTakeHostage,
    PhysicallyAssault(PhysicallyAssault),
    Bombing(Bombing),
    HumanShield,
    AssasinationAttempt,
    Assasinate,
}

pub enum Bombing {
    NotSpecified,
    Suicide,
    Vehicular,
    Roadside,
    Location,
}

pub enum PhysicallyAssault {
    NotSpecified,
    SexualAssault,
    Torture,
    Murder,
}
