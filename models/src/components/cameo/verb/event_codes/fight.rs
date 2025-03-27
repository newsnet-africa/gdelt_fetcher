pub enum Fight {
    NotSpecified,
    BlockadeRestrictMovement,
    OccupyTeritory,
    SmallArmsAndLightWeapons,
    ArtilleryAndTanks,
    ArialWeapons(ArialWeapons),
    ViolateCeasefire,
}

pub enum ArialWeapons {
    PrecisionGuided,
    RemotelyPiloted,
}
