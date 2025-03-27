use url::Url;

pub enum EntityType {
    Unknown,
    Person,
    Location,
    Organisation,
    Event,
    WorkOfArt,
    ConsumerGood,
    PhoneNumber,
    Address,
    Date,
    Number,
    Price,
}

pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub mid: Option<String>,
    pub wikipedia_url: Option<Url>,
    pub num_mentions: u128,
    pub avg_salience: f64,
}
