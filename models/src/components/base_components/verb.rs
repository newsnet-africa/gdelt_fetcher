pub mod top_level_actions {
    pub enum Verb {
        MakePublicStatement(PublicStatement),
        Appeal(AppealGoal),
        IntentionToCooperate(Cooperation),
        Consult(Consultation),
        EngageInDiplomaticCooperation(DiplomaticCooperation),
        EngageInMaterialCooperation(MaterialCooperation),
        ProvideAid(Aid),
        Yield(Yieldable),
        Investigate(Investigation),
        Demand(Demand),
        Disapprove(Disapproval),
        Reject(Rejection),
        Threaten(Threat),
        Protest(Protest),
        ExhibitForcePosture(ForcePosture),
        ReduceRelations(Relations),
        Coerce(Coercion),
        Assault(Assault),
        Fight(Fight),
        UseUnconventionalMassViolence(MassViolence),
    }
}

pub mod subcategories {
    pub enum PublicStatement {}
}
