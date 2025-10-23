use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// GEG (GDELT Event Graph) table entry from GDELT 3.0
///
/// The GEG-GCNLAPI dataset processes news articles through Google's Cloud Natural Language API.
/// Each entry represents a single article with entity extraction and sentiment analysis.
///
/// Data format: Newline-delimited JSON
/// URL pattern: http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/YYYYMMDDHHMMSS.geg-gcnlapi.json.gz
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GEGTable {
    /// Timestamp the article was seen, rounded to nearest 15 minutes
    pub date: NaiveDateTime,

    /// URL of the article
    pub url: String,

    /// Google-provided language code used by the API
    pub lang: String,

    /// Article sentiment polarity (deprecated by API, may be None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polarity: Option<f64>,

    /// Article sentiment magnitude
    pub magnitude: f64,

    /// Article sentiment score (added later, not present in early dataset)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,

    /// List of entities identified by the Natural Language API
    pub entities: Vec<GEGEntity>,
}

impl GEGTable {
    /// Parse a GEG entry from a JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Parse multiple GEG entries from newline-delimited JSON
    ///
    /// This handles the GEG format which may have:
    /// - Single JSON object per line
    /// - Multiple JSON objects per line (space-separated)
    /// - Trailing whitespace or extra data
    pub fn from_ndjson(ndjson: &str) -> Result<Vec<Self>, String> {
        let mut entries = Vec::new();
        let mut warnings = 0;
        const MAX_WARNINGS: usize = 10;
        const DEBUG: bool = true; // Enable detailed debugging

        let total_lines = ndjson.lines().count();
        if DEBUG {
            eprintln!("DEBUG: Parsing NDJSON with {} total lines", total_lines);
        }

        for (i, line) in ndjson.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                if DEBUG && i < 5 {
                    eprintln!("DEBUG: Line {} is empty, skipping", i + 1);
                }
                continue;
            }

            if DEBUG && i < 3 {
                let preview = if line.len() > 100 {
                    format!("{}...", &line[..100])
                } else {
                    line.to_string()
                };
                eprintln!("DEBUG: Line {} (len={}): {}", i + 1, line.len(), preview);
            }

            // Create a deserializer from the line
            let mut deserializer = serde_json::Deserializer::from_str(line);

            // Try to deserialize, ignoring trailing data
            match GEGTable::deserialize(&mut deserializer) {
                Ok(entry) => {
                    if DEBUG && entries.len() < 3 {
                        eprintln!("DEBUG: Successfully parsed entry {} from line {}", entries.len() + 1, i + 1);
                    }
                    entries.push(entry);
                    // Note: We ignore trailing data after the first valid JSON object
                }
                Err(e) => {
                    if warnings < MAX_WARNINGS {
                        eprintln!("Warning: Failed to parse JSON line {}: {}", i + 1, e);
                        if DEBUG && warnings < 3 {
                            // Show more context for first few errors
                            let line_preview = if line.len() > 200 {
                                format!("{}...", &line[..200])
                            } else {
                                line.to_string()
                            };
                            eprintln!("  Line content: {}", line_preview);
                        }
                        warnings += 1;
                    } else if warnings == MAX_WARNINGS {
                        eprintln!("Warning: Suppressing further parsing warnings...");
                        warnings += 1;
                    }
                }
            }
        }

        if DEBUG {
            eprintln!("DEBUG: Successfully parsed {} entries out of {} lines", entries.len(), total_lines);
        }

        if warnings > MAX_WARNINGS {
            eprintln!("Total {} lines failed to parse (showing only first {} errors)", warnings, MAX_WARNINGS);
        }

        Ok(entries)
    }

    /// Get all entities of a specific type
    pub fn entities_by_type(&self, entity_type: &GEGEntityType) -> Vec<&GEGEntity> {
        self.entities
            .iter()
            .filter(|e| &e.entity_type == entity_type)
            .collect()
    }

    /// Get all entities with a Wikipedia URL
    pub fn entities_with_wikipedia(&self) -> Vec<&GEGEntity> {
        self.entities
            .iter()
            .filter(|e| e.wikipedia_url.is_some())
            .collect()
    }

    /// Get all entities with a MID (Google Knowledge Graph ID)
    pub fn entities_with_mid(&self) -> Vec<&GEGEntity> {
        self.entities
            .iter()
            .filter(|e| e.mid.is_some())
            .collect()
    }

    /// Get the most salient entities (top N by average salience)
    pub fn top_entities(&self, n: usize) -> Vec<&GEGEntity> {
        let mut entities = self.entities.iter().collect::<Vec<_>>();
        entities.sort_by(|a, b| {
            b.avg_salience
                .partial_cmp(&a.avg_salience)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        entities.into_iter().take(n).collect()
    }

    /// Calculate statistics about the entities
    pub fn entity_stats(&self) -> GEGEntityStats {
        let total_entities = self.entities.len();
        let entities_with_mid = self.entities_with_mid().len();
        let entities_with_wikipedia = self.entities_with_wikipedia().len();

        let total_mentions: usize = self.entities.iter().map(|e| e.num_mentions).sum();

        let avg_salience = if !self.entities.is_empty() {
            self.entities.iter().map(|e| e.avg_salience).sum::<f64>() / self.entities.len() as f64
        } else {
            0.0
        };

        GEGEntityStats {
            total_entities,
            entities_with_mid,
            entities_with_wikipedia,
            total_mentions,
            avg_salience,
        }
    }
}

/// Entity identified by the Natural Language API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GEGEntity {
    /// Entity name as identified by the API
    pub name: String,

    /// Entity type (PERSON, LOCATION, ORGANIZATION, etc.)
    #[serde(rename = "type")]
    pub entity_type: GEGEntityType,

    /// Google Knowledge Graph ID (only for well-known entities)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<String>,

    /// Wikipedia URL (only for well-known entities)
    #[serde(rename = "wikipediaUrl", skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<String>,

    /// Number of times this entity+type was mentioned in the article
    #[serde(rename = "numMentions")]
    pub num_mentions: usize,

    /// Average salience score for this entity
    #[serde(rename = "avgSalience")]
    pub avg_salience: f64,
}

impl GEGEntity {
    /// Check if this is a well-known entity (has MID and/or Wikipedia URL)
    pub fn is_well_known(&self) -> bool {
        self.mid.is_some() || self.wikipedia_url.is_some()
    }

    /// Get a human-readable description of the entity
    pub fn description(&self) -> String {
        format!(
            "{} ({:?}) - {} mentions, salience: {:.3}",
            self.name, self.entity_type, self.num_mentions, self.avg_salience
        )
    }
}

/// Entity types recognized by the Natural Language API
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum GEGEntityType {
    Person,
    Location,
    Organization,
    Event,
    #[serde(rename = "WORK_OF_ART")]
    WorkOfArt,
    #[serde(rename = "CONSUMER_GOOD")]
    ConsumerGood,
    Other,
    #[serde(rename = "PHONE_NUMBER")]
    PhoneNumber,
    Address,
    Date,
    Number,
    Price,
    Unknown,
}

impl GEGEntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GEGEntityType::Person => "PERSON",
            GEGEntityType::Location => "LOCATION",
            GEGEntityType::Organization => "ORGANIZATION",
            GEGEntityType::Event => "EVENT",
            GEGEntityType::WorkOfArt => "WORK_OF_ART",
            GEGEntityType::ConsumerGood => "CONSUMER_GOOD",
            GEGEntityType::Other => "OTHER",
            GEGEntityType::PhoneNumber => "PHONE_NUMBER",
            GEGEntityType::Address => "ADDRESS",
            GEGEntityType::Date => "DATE",
            GEGEntityType::Number => "NUMBER",
            GEGEntityType::Price => "PRICE",
            GEGEntityType::Unknown => "UNKNOWN",
        }
    }
}

impl std::fmt::Display for GEGEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Statistics about entities in a GEG entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GEGEntityStats {
    pub total_entities: usize,
    pub entities_with_mid: usize,
    pub entities_with_wikipedia: usize,
    pub total_mentions: usize,
    pub avg_salience: f64,
}

impl std::fmt::Display for GEGEntityStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Entities: {} total, {} with MID, {} with Wikipedia, {} total mentions, avg salience: {:.3}",
            self.total_entities,
            self.entities_with_mid,
            self.entities_with_wikipedia,
            self.total_mentions,
            self.avg_salience
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_geg_entry() {
        let json = r#"{
            "date": "2020-01-17T12:00:00",
            "url": "https://example.com/article",
            "lang": "en",
            "magnitude": 0.8,
            "score": 0.2,
            "entities": [
                {
                    "name": "White House",
                    "type": "LOCATION",
                    "mid": "/m/081sq",
                    "wikipediaUrl": "https://en.wikipedia.org/wiki/White_House",
                    "numMentions": 5,
                    "avgSalience": 0.75
                },
                {
                    "name": "President",
                    "type": "PERSON",
                    "numMentions": 3,
                    "avgSalience": 0.65
                }
            ]
        }"#;

        let entry: GEGTable = serde_json::from_str(json).unwrap();

        assert_eq!(entry.url, "https://example.com/article");
        assert_eq!(entry.lang, "en");
        assert_eq!(entry.magnitude, 0.8);
        assert_eq!(entry.score, Some(0.2));
        assert_eq!(entry.entities.len(), 2);

        let white_house = &entry.entities[0];
        assert_eq!(white_house.name, "White House");
        assert_eq!(white_house.entity_type, GEGEntityType::Location);
        assert_eq!(white_house.mid, Some("/m/081sq".to_string()));
        assert_eq!(white_house.num_mentions, 5);

        let president = &entry.entities[1];
        assert_eq!(president.name, "President");
        assert_eq!(president.entity_type, GEGEntityType::Person);
        assert_eq!(president.mid, None);
    }

    #[test]
    fn test_entities_by_type() {
        let json = r#"{
            "date": "2020-01-17T12:00:00",
            "url": "https://example.com/article",
            "lang": "en",
            "magnitude": 0.8,
            "entities": [
                {
                    "name": "White House",
                    "type": "LOCATION",
                    "numMentions": 5,
                    "avgSalience": 0.75
                },
                {
                    "name": "President",
                    "type": "PERSON",
                    "numMentions": 3,
                    "avgSalience": 0.65
                },
                {
                    "name": "Washington DC",
                    "type": "LOCATION",
                    "numMentions": 2,
                    "avgSalience": 0.55
                }
            ]
        }"#;

        let entry: GEGTable = serde_json::from_str(json).unwrap();

        let locations = entry.entities_by_type(&GEGEntityType::Location);
        assert_eq!(locations.len(), 2);

        let persons = entry.entities_by_type(&GEGEntityType::Person);
        assert_eq!(persons.len(), 1);
    }

    #[test]
    fn test_top_entities() {
        let json = r#"{
            "date": "2020-01-17T12:00:00",
            "url": "https://example.com/article",
            "lang": "en",
            "magnitude": 0.8,
            "entities": [
                {
                    "name": "Entity1",
                    "type": "PERSON",
                    "numMentions": 5,
                    "avgSalience": 0.9
                },
                {
                    "name": "Entity2",
                    "type": "LOCATION",
                    "numMentions": 3,
                    "avgSalience": 0.7
                },
                {
                    "name": "Entity3",
                    "type": "ORGANIZATION",
                    "numMentions": 2,
                    "avgSalience": 0.5
                }
            ]
        }"#;

        let entry: GEGTable = serde_json::from_str(json).unwrap();

        let top2 = entry.top_entities(2);
        assert_eq!(top2.len(), 2);
        assert_eq!(top2[0].name, "Entity1");
        assert_eq!(top2[1].name, "Entity2");
    }

    #[test]
    fn test_entity_stats() {
        let json = r#"{
            "date": "2020-01-17T12:00:00",
            "url": "https://example.com/article",
            "lang": "en",
            "magnitude": 0.8,
            "entities": [
                {
                    "name": "White House",
                    "type": "LOCATION",
                    "mid": "/m/081sq",
                    "wikipediaUrl": "https://en.wikipedia.org/wiki/White_House",
                    "numMentions": 5,
                    "avgSalience": 0.8
                },
                {
                    "name": "President",
                    "type": "PERSON",
                    "numMentions": 3,
                    "avgSalience": 0.6
                }
            ]
        }"#;

        let entry: GEGTable = serde_json::from_str(json).unwrap();
        let stats = entry.entity_stats();

        assert_eq!(stats.total_entities, 2);
        assert_eq!(stats.entities_with_mid, 1);
        assert_eq!(stats.entities_with_wikipedia, 1);
        assert_eq!(stats.total_mentions, 8);
        assert!((stats.avg_salience - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_parse_with_trailing_data() {
        // Test that we can parse JSON with trailing data
        let line = r#"{"date":"2020-01-17T12:00:00","url":"https://example.com","lang":"en","magnitude":0.8,"entities":[]} some trailing data here"#;

        let mut deserializer = serde_json::Deserializer::from_str(line);
        let entry = GEGTable::deserialize(&mut deserializer);

        assert!(entry.is_ok(), "Should parse JSON even with trailing data");
        let entry = entry.unwrap();
        assert_eq!(entry.url, "https://example.com");
        assert_eq!(entry.lang, "en");
    }

    #[test]
    fn test_ndjson_with_trailing_data() {
        let ndjson = r#"{"date":"2020-01-17T12:00:00","url":"https://example1.com","lang":"en","magnitude":0.8,"entities":[]} trailing1
{"date":"2020-01-17T12:00:00","url":"https://example2.com","lang":"en","magnitude":0.9,"entities":[]} trailing2
{"date":"2020-01-17T12:00:00","url":"https://example3.com","lang":"en","magnitude":0.7,"entities":[]}"#;

        let entries = GEGTable::from_ndjson(ndjson).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].url, "https://example1.com");
        assert_eq!(entries[1].url, "https://example2.com");
        assert_eq!(entries[2].url, "https://example3.com");
    }

    #[test]
    fn test_entity_types_with_underscores() {
        let json = r#"{
            "date": "2020-01-17T12:00:00",
            "url": "https://example.com/article",
            "lang": "en",
            "magnitude": 0.8,
            "entities": [
                {
                    "name": "Mona Lisa",
                    "type": "WORK_OF_ART",
                    "numMentions": 3,
                    "avgSalience": 0.75
                },
                {
                    "name": "iPhone",
                    "type": "CONSUMER_GOOD",
                    "numMentions": 2,
                    "avgSalience": 0.65
                }
            ]
        }"#;

        let entry: GEGTable = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entities.len(), 2);
        assert_eq!(entry.entities[0].entity_type, GEGEntityType::WorkOfArt);
        assert_eq!(entry.entities[1].entity_type, GEGEntityType::ConsumerGood);
    }
}
