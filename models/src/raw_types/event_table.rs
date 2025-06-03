pub struct GlobalEventID<'a>(pub &'a str);
pub struct YearMonthDay<'a>(pub &'a str);
pub struct YearMonth<'a>(pub &'a str);
pub struct Year<'a>(pub &'a str);
pub struct FractionDate<'a>(pub &'a str);
pub struct YearMonthDayHourMinuteSecond<'a>(pub &'a str);

pub mod actor {
    pub struct CAMEOActorCode<'a>(pub &'a str);
    pub struct ActorName<'a>(pub &'a str);
    pub struct CAMEOCountryCode<'a>(pub &'a str);
    pub struct CAMEOcKnownGroupCode<'a>(pub &'a str);
    pub struct CAMEOReligionCode<'a>(pub &'a str);
    pub struct CAMEORoleCode<'a>(pub &'a str);
}

pub mod event_action {
    pub struct IsRootEvent<'a>(pub &'a str);
    pub struct CAMEOEventCode<'a>(pub &'a str);
    pub struct CAMEOEventBaseCode<'a>(pub &'a str);
    pub struct CAMEOEventRootCode<'a>(pub &'a str);
    pub struct QuadClassCode<'a>(pub &'a str);
    pub struct GoldsteinScale<'a>(pub &'a str);
    pub struct NumberOfMentions<'a>(pub &'a str);
    pub struct NumberOfSources<'a>(pub &'a str);
    pub struct NumberOfArticles<'a>(pub &'a str);
    pub struct AverageTone<'a>(pub &'a str);
}

pub mod event_geography {
    pub struct GeograpyTypeCode<'a>(pub &'a str);
    pub struct GeographyFullName<'a>(pub &'a str);
    pub struct FIPSCountryCode<'a>(pub &'a str);
    pub struct FIPSAdministrationCode<'a>(pub &'a str);
    pub struct Administration2Code<'a>(pub &'a str);
    pub struct Latitude<'a>(pub &'a str);
    pub struct Longitude<'a>(pub &'a str);
    pub struct FeatureID<'a>(pub &'a str);
}
