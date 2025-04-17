pub struct Entity {
    pub name: String,
    pub entity_type: String,
    pub mid: Option<String>,
    pub wikipedia_url: Option<String>,
    pub num_mentions: u32,
    pub avg_salience: f64,
}

pub struct GlobalEntityGraphTable {
    pub date: String,
    pub url: String,
    pub lang: String,
    pub polarity: Option<f64>,
    pub magnitude: Option<f64>,
    pub score: Option<f64>,
    pub entities: Vec<Entity>,
}
