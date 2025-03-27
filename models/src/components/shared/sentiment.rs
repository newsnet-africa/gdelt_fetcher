#[repr(transparent)]
pub struct Polarity(bool);

pub struct Sentiment {
    pub polarity: Polarity,
    pub score: f32,
    pub magnitude: u128,
}
