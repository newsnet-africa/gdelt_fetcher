pub struct ToneValue(f64);

pub enum ToneScore {
    Positive(f32),
    Negative(f32),
}

pub struct Polarity(f64);

pub struct ActivityReferenceDensity(f64);

pub struct SelfGroupReferenceDensity(f64);

pub struct WordCount(u128);

pub struct Tone {
    pub tone: ToneValue,
    pub tone_score: ToneScore,
    pub polarity: Polarity,
    pub activity_reference_density: ActivityReferenceDensity,
    pub self_group_reference_density: SelfGroupReferenceDensity,
    pub word_count: WordCount,
}
