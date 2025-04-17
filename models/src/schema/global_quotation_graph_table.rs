// Represents a quotation identified in the article
#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub pre: String,   // Snippet of text preceding the quotation
    pub quote: String, // The actual quoted statement
    pub post: String,  // Snippet of text following the quotation
}

// Represents an article with its details
#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub date: String,       // Date and time the article was seen
    pub url: String,        // Full URL of the article
    pub title: String,      // Title of the article
    pub lang: String,       // Human-readable name of the language
    pub quotes: Vec<Quote>, // Array of quotations identified in the article
}
