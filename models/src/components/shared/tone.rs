use super::sentiment::Sentiment;

pub struct Tone {
    pub sentiment: Sentiment,
    pub activity_reference_density: f64,
    pub pronoun_reference_density: f64,
    pub word_count: u128,
}
