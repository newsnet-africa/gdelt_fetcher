// region: Imports
use super::utils::gdelt_categorylist::GDELTCategoryList;
use crate::generated::gdelt_count::GdeltCount;
use crate::generated::gkg::global_knowledge_graph::{Amount, NameWithOffset, OrganisationWithOffset, PersonWithOffset, ThemeWithOffset, TranslationInfo};
use super::utils::gdelt_counts::GDELTCount;
use super::utils::gdelt_date::GDELTDate;
use super::utils::gdelt_location::GDELTLocation;
use super::utils::gdelt_quotation::GDELTQuotation;
use super::utils::gdelt_source_collection_identifier::SourceCollectionIdentifier;
use super::utils::gdelt_tone::Tone;
use crate::models::gdelt::{CellItem, DatabaseTableEntry, GDELTObject, ToProto};
use crate::generated::gkg::GlobalKnowledgeGraph as GdeltGlobalKnowledgeGraph;
// endregion

/// \[GlobalKnowledgeGraph\] represents a record in the Global Knowledge Graph (GKG) dataset.
///
/// This struct contains various fields that correspond to the attributes of a GKG record.
/// Each field is an \[Option\] type to handle the possibility of missing data.
#[derive(Debug, Clone)]
pub struct GlobalKnowledgeGraph {
    record_id: Option<String>,
    v2_1_date: Option<GDELTDate>,
    source_collection_identifier: Option<SourceCollectionIdentifier>,
    source_common_name: Option<String>,
    document_identifier: Option<String>,
    counts: (Option<Vec<GDELTCount>>, Option<Vec<GDELTCount>>),
    themes: (
        Option<Vec<GDELTCategoryList>>,
        Option<Vec<(GDELTCategoryList, Option<u64>)>>,
    ),
    locations: (Option<Vec<GDELTLocation>>, Option<Vec<GDELTLocation>>),
    persons: (Option<Vec<String>>, Option<Vec<(String, u128)>>),
    organisations: (Option<Vec<String>>, Option<Vec<(String, u128)>>),
    tone: Option<Tone>,
    enhanced_dates: Option<Vec<GDELTDate>>,
    sharing_image: Option<String>,
    related_images: Option<Vec<String>>,
    social_image_embeds: Option<Vec<String>>,
    social_video_embeds: Option<Vec<String>>,
    quotations: Option<Vec<GDELTQuotation>>,
    all_names: Option<Vec<(String, u128)>>,
    amounts: Option<Vec<(i128, String, u128)>>,
    translation_info: Option<(String, String)>,
}

impl ToProto for GlobalKnowledgeGraph {
    type ProtoType = GdeltGlobalKnowledgeGraph;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        if self.record_id.is_none()
            && self.v2_1_date.is_none()
            && self.source_collection_identifier.is_none()
            && self.source_common_name.is_none()
            && self.document_identifier.is_none()
            && self.counts.0.is_none()
            && self.counts.1.is_none()
            && self.themes.0.is_none()
            && self.themes.1.is_none()
            && self.locations.0.is_none()
            && self.locations.1.is_none()
            && self.persons.0.is_none()
            && self.persons.1.is_none()
            && self.organisations.0.is_none()
            && self.organisations.1.is_none()
            && self.tone.is_none()
            && self.enhanced_dates.is_none()
            && self.sharing_image.is_none()
            && self.related_images.is_none()
            && self.social_image_embeds.is_none()
            && self.social_video_embeds.is_none()
            && self.quotations.is_none()
            && self.all_names.is_none()
            && self.amounts.is_none()
            && self.translation_info.is_none()
        {
            None
        } else {
            Some(GdeltGlobalKnowledgeGraph {
                record_id: self.record_id.clone(),
                v2_1_date: self.v2_1_date.as_ref().and_then(|date| date.to_proto()),
                source_collection_identifier: self.source_collection_identifier.clone().map(|sci| sci as i32),
                source_common_name: self.source_common_name.clone(),
                document_identifier: self.document_identifier.clone(),
                counts_v1: match self.counts.0.clone() {
                    Some(counts) => counts.iter().filter_map(|cou| cou.to_proto()).collect(),
                    None => vec![],
                },
                counts_v2: match self.counts.1.clone() {
                    Some(counts) => counts.iter().filter_map(|cou| cou.to_proto()).collect(),
                    None => vec![],
                },
                themes_v1: match self.themes.0.clone() {
                    Some(themes) => themes.iter().map(|categ| u16::from(categ.clone()) as i32).collect(),
                    None => vec![],
                },
                themes_v2: match self.themes.1.clone() {
                    Some(themes) => themes.iter().map(|categ| 
                        ThemeWithOffset {
                            theme: u16::from(categ.clone().0) as i32,
                            offset: categ.1,
                    }).collect(),
                    None => vec![],
                },
                locations_v1: match self.locations.0.clone() {
                    Some(locations) => locations.iter().filter_map(|l| l.to_proto()).collect(),
                    None => vec![],
                },
                locations_v2: match self.locations.1.clone() {
                    Some(locations) => locations.iter().filter_map(|l| l.to_proto()).collect(),
                    None => vec![],
                },
                persons_v1: self.persons.0.clone().unwrap_or_else(|| vec![]),
                persons_v2: match self.persons.1.clone() {
                    Some(persons) => persons.iter().map(|(person, offset)| PersonWithOffset {
                        person: person.clone(),
                        offset: offset.clone() as u64,
                    }).collect(),
                    None => vec![],
                },
                organisations_v1: self.organisations.0.clone().unwrap_or_else(|| vec![]),
                organisations_v2: match self.organisations.1.clone() {
                    Some(organisations) => organisations.iter().map(|(org, offset)| {
                        OrganisationWithOffset {
                            organisation: org.clone(),
                            offset: offset.clone() as u64,
                        }
                    }).collect(),
                    None => vec![],
                },
                tone: None,
                enhanced_dates: match self.enhanced_dates.clone() {
                    Some(dates) => dates.iter().filter_map(|d| d.to_proto()).collect(),
                    None => vec![],
                },
                sharing_image: None,
                related_images: vec![],
                social_image_embeds: vec![],
                social_video_embeds: vec![],
                quotations: match self.quotations.clone() {
                    Some(quotations) => quotations.iter().filter_map(|q| q.to_proto()).collect(),
                    None => vec![],
                },
                all_names: match self.all_names.clone() {
                    Some(names) => names.iter().map(|(name, offset)| {
                        NameWithOffset {
                            name: name.clone(),
                            offset: offset.clone() as u64,
                        }
                    }).collect(),
                    None => vec![],
                },
                amounts: match self.amounts.clone() {
                    Some(amounts) => amounts.iter().map(|(value, currency, count)| {
                        Amount {
                            value: value.clone() as i64,
                            currency: currency.clone(),
                            count: count.clone() as u64,
                        }
                    }).collect(),
                    None => vec![],
                },translation_info: self.translation_info.clone().map(|(language, translation)| {
                    TranslationInfo {
                        language,
                        translation,
                    }
                }),
            })
        }
    }
}

// region: GlobalKnowledgeGraph Implementation
impl GlobalKnowledgeGraph {
    /// Returns a reference to the \[record_id\] field.
    pub fn record_id(&self) -> &Option<String> {
        &self.record_id
    }

    /// Returns a reference to the \[v2_1_date\] field.
    pub fn v2_1_date(&self) -> &Option<GDELTDate> {
        &self.v2_1_date
    }

    /// Returns a reference to the \[source_collection_identifier\] field.
    pub fn source_collection_identifier(&self) -> &Option<SourceCollectionIdentifier> {
        &self.source_collection_identifier
    }

    /// Returns a reference to the \[source_common_name\] field.
    pub fn source_common_name(&self) -> &Option<String> {
        &self.source_common_name
    }

    /// Returns a reference to the \[document_identifier\] field.
    pub fn document_identifier(&self) -> &Option<String> {
        &self.document_identifier
    }

    /// Returns a reference to the \[counts\] field.
    pub fn counts(&self) -> &(Option<Vec<GDELTCount>>, Option<Vec<GDELTCount>>) {
        &self.counts
    }

    /// Returns a reference to the \[themes\] field.
    pub fn themes(
        &self,
    ) -> &(
        Option<Vec<GDELTCategoryList>>,
        Option<Vec<(GDELTCategoryList, Option<u64>)>>,
    ) {
        &self.themes
    }

    /// Returns a reference to the \[persons\] field.
    pub fn persons(&self) -> &(Option<Vec<String>>, Option<Vec<(String, u128)>>) {
        &self.persons
    }

    /// Returns a reference to the \[organisations\] field.
    pub fn organisations(&self) -> &(Option<Vec<String>>, Option<Vec<(String, u128)>>) {
        &self.organisations
    }

    /// Returns a reference to the \[tone\] field.
    pub fn tone(&self) -> &Option<Tone> {
        &self.tone
    }

    /// Returns a reference to the \[enhanced_dates\] field.
    pub fn enhanced_dates(&self) -> &Option<Vec<GDELTDate>> {
        &self.enhanced_dates
    }

    /// Returns a reference to the \[sharing_image\] field.
    pub fn sharing_image(&self) -> &Option<String> {
        &self.sharing_image
    }

    /// Returns a reference to the \[related_images\] field.
    pub fn related_images(&self) -> &Option<Vec<String>> {
        &self.related_images
    }

    /// Returns a reference to the \[social_image_embeds\] field.
    pub fn social_image_embeds(&self) -> &Option<Vec<String>> {
        &self.social_image_embeds
    }

    /// Returns a reference to the \[social_video_embeds\] field.
    pub fn social_video_embeds(&self) -> &Option<Vec<String>> {
        &self.social_video_embeds
    }

    /// Returns a reference to the \[quotations\] field.
    pub fn quotations(&self) -> &Option<Vec<GDELTQuotation>> {
        &self.quotations
    }

    /// Returns a reference to the \[amounts\] field.
    pub fn amounts(&self) -> &Option<Vec<(i128, String, u128)>> {
        &self.amounts
    }

    /// Returns a reference to the \[translation_info\] field.
    pub fn translation_info(&self) -> &Option<(String, String)> {
        &self.translation_info
    }

    /// Returns a reference to the \[locations\] field.
    pub fn locations(&self) -> &(Option<Vec<GDELTLocation>>, Option<Vec<GDELTLocation>>) {
        &self.locations
    }

    /// Returns a reference to the \[all_names\] field.
    pub fn all_names(&self) -> &Option<Vec<(String, u128)>> {
        &self.all_names
    }
}
// endregion

// region: DatabaseTableEntry Implementation
impl DatabaseTableEntry for GlobalKnowledgeGraph {
    /// Creates a blank \[GlobalKnowledgeGraph\] instance with all fields set to \[None\].
    fn blank() -> Self {
        Self {
            record_id: None,
            v2_1_date: None,
            source_collection_identifier: None,
            source_common_name: None,
            document_identifier: None,
            counts: (None, None),
            themes: (None, None),
            locations: (None, None),
            persons: (None, None),
            organisations: (None, None),
            tone: None,
            enhanced_dates: None,
            sharing_image: None,
            related_images: None,
            social_image_embeds: None,
            social_video_embeds: None,
            quotations: None,
            all_names: None,
            amounts: None,
            translation_info: None,
        }
    }

    /// Creates a \[GlobalKnowledgeGraph\] instance from a CSV row.
    ///
    /// # Arguments
    ///
    /// * \[row\] - A string slice that holds the CSV row.
    ///
    /// # Returns
    ///
    /// * \[Option<Self>\] - An optional \[GlobalKnowledgeGraph\] instance. Returns \[None\] if parsing fails.
    fn from_csv_row(row: &str) -> Option<Self> {
        let fields = <GlobalKnowledgeGraph as GDELTObject>::delimited_vector("\t", row);
        Self::new(fields)
    }
}
// endregion

// region: GDELTObject Implementation
impl GDELTObject for GlobalKnowledgeGraph {
    /// Creates a \[GlobalKnowledgeGraph\] instance from a delimited string.
    ///
    /// # Arguments
    ///
    /// * \[record\] - A string slice that holds the delimited event attributes.
    ///
    /// # Returns
    ///
    /// * \[Option<Self>\] - An optional \[GlobalKnowledgeGraph\] instance. Returns \[None\] if parsing fails.
    fn from_strings(record: &str) -> Option<Self> {
        let fields = <GlobalKnowledgeGraph as GDELTObject>::delimited_vector("\t", record);

        Self::new(fields)
    }

    /// Creates a new \[GlobalKnowledgeGraph\] instance from a vector of string fields.
    ///
    /// # Arguments
    ///
    /// * \[fields\] - A vector of string slices representing the fields of the event.
    ///
    /// # Returns
    ///
    /// * \[Option<Self>\] - An optional \[GlobalKnowledgeGraph\] instance. Returns \[None\] if all fields are default values.
    fn new(fields: Vec<&str>) -> Option<Self> {
        // Parse the record ID field.
        let record_id = match fields.get(0) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        // Parse the GDELT date field.
        let v2_1_date = match fields.get(1) {
            Some(&"") | None => None,
            Some(value) => GDELTDate::from_strings(value),
        };

        // Parse the source collection identifier field.
        let source_collection_identifier = match fields.get(2) {
            Some(&"") | None => None,
            Some(value) => SourceCollectionIdentifier::from_string(value),
        };

        // Parse the source common name field.
        let source_common_name = match fields.get(3) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        // Parse the document identifier field.
        let document_identifier = match fields.get(4) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        // Parse the counts fields.
        let counts_v1 = match fields.get(5) {
            Some(&"") | None => None,
            Some(value) => GDELTCount::vec_from_cell(value),
        };

        let counts_v2 = match fields.get(6) {
            Some(&"") | None => None,
            Some(value) => GDELTCount::vec_from_cell(value),
        };

        // Parse the themes fields.
        let themes_v1 = match fields.get(7) {
            Some(&"") | None => None,
            Some(value) => GDELTCategoryList::vec_from_cell(value),
        };

        let themes_v2 = match fields.get(8) {
            Some(&"") | None => None,
            Some(value) => match GDELTCategoryList::vec_from_cell(value) {
                Some(v) => Some(
                    v.iter()
                        .map(|categ| {
                            let offset = GDELTCategoryList::extract_value(categ.clone());
                            (categ.clone(), offset)
                        })
                        .collect::<Vec<(GDELTCategoryList, Option<u64>)>>(),
                ),
                None => None,
            },
        };

        // Parse the locations fields.
        let locations_v1 = match fields.get(9) {
            Some(&"") | None => None,
            Some(value) => GDELTLocation::vec_from_cell(value),
        };

        let locations_v2 = match fields.get(10) {
            Some(&"") | None => None,
            Some(value) => GDELTLocation::vec_from_cell(value),
        };

        // Parse the persons fields.
        let persons_v1 = match fields.get(11) {
            Some(&"") | None => None,
            Some(value) => Some(
                <Self as GDELTObject>::delimited_vector(";", value)
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        };

        let persons_v2 = match fields.get(12) {
            Some(&"") | None => None,
            Some(value) => {
                let chunks = <Self as GDELTObject>::delimited_vector(";", value);
                Some(
                    chunks
                        .iter()
                        .filter_map(|ch| {
                            let chunk_vec = <Self as GDELTObject>::delimited_vector(",", ch);
                            match chunk_vec.get(1) {
                                None => Some((ch.to_string(), 0)),
                                Some(offset) => match offset.parse::<u128>() {
                                    Ok(u_128) => Some((ch.to_string(), u_128)),
                                    Err(_) => None,
                                },
                            }
                        })
                        .collect::<Vec<(String, u128)>>(),
                )
            }
        };

        // Parse the organisations fields.
        let organisations_v1 = match fields.get(13) {
            Some(&"") | None => None,
            Some(value) => Some(
                <Self as GDELTObject>::delimited_vector(";", value)
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        };

        let organisations_v2 = match fields.get(14) {
            Some(&"") | None => None,
            Some(value) => {
                let chunks = <Self as GDELTObject>::delimited_vector(";", value);
                Some(
                    chunks
                        .iter()
                        .filter_map(|ch| {
                            let chunk_vec = <Self as GDELTObject>::delimited_vector(",", ch);
                            match (chunk_vec.get(0), chunk_vec.get(1)) {
                                (Some(name), Some(offset)) => match offset.parse::<u128>() {
                                    Ok(parsed_offset) => Some((name.to_string(), parsed_offset)),
                                    Err(_) => None,
                                },
                                _ => None,
                            }
                        })
                        .collect(),
                )
            }
        };

        // Parse the tone field.
        let tone = match fields.get(15) {
            Some(&"") | None => None,
            Some(value) => Tone::from_strings(value),
        };

        // Parse the enhanced dates field.
        let enhanced_dates = match fields.get(16) {
            Some(&"") | None => None,
            Some(value) => GDELTDate::vec_from_cell(value),
        };

        // Parse the sharing image field.
        let sharing_image = match fields.get(18) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        // Parse the related images field.
        let related_images = match fields.get(19) {
            Some(&"") | None => None,
            Some(value) => {
                let v = <Self as GDELTObject>::delimited_vector(";", value);
                Some(v.iter().map(|s| s.to_string()).collect())
            }
        };

        // Parse the social image embeds field.
        let social_image_embeds = match fields.get(20) {
            Some(&"") | None => None,
            Some(value) => {
                let v = <Self as GDELTObject>::delimited_vector(";", value);
                Some(v.iter().map(|s| s.to_string()).collect())
            }
        };

        // Parse the social video embeds field.
        let social_video_embeds = match fields.get(21) {
            Some(&"") | None => None,
            Some(value) => {
                let v = <Self as GDELTObject>::delimited_vector(";", value);
                Some(v.iter().map(|s| s.to_string()).collect())
            }
        };

        // Parse the quotations field.
        let quotations = match fields.get(22) {
            Some(&"") | None => None,
            Some(value) => {
                let v = <Self as GDELTObject>::delimited_vector("#", value);
                Some(
                    v.iter()
                        .filter_map(|s| GDELTQuotation::from_strings(s))
                        .collect(),
                )
            }
        };

        // Parse the all names field.
        let all_names = match fields.get(23) {
            Some(&"") | None => None,
            Some(value) => {
                let v = <Self as GDELTObject>::delimited_vector(";", value);
                Some(
                    v.iter()
                        .map(|s| {
                            let names =
                                s.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
                            (
                                names.get(0).cloned().unwrap_or_default(),
                                names
                                    .get(1)
                                    .and_then(|s| s.parse::<u128>().ok())
                                    .unwrap_or_default(),
                            )
                        })
                        .collect::<Vec<(String, u128)>>(),
                )
            }
        };

        // Parse the amounts field.
        let amounts = match fields.get(24) {
            Some(&"") | None => None,
            Some(value) => {
                let amount_parts = <Self as GDELTObject>::delimited_vector(";", value);
                Some(
                    amount_parts
                        .iter()
                        .filter_map(|amount| {
                            let newv = <Self as GDELTObject>::delimited_vector(",", amount);
                            if let (Some(value), Some(currency), Some(count)) =
                                (newv.get(0), newv.get(1), newv.get(2))
                            {
                                Some((
                                    value.parse::<i128>().unwrap_or(0),
                                    currency.to_string(),
                                    count.parse::<u128>().unwrap_or(0),
                                ))
                            } else {
                                None
                            }
                        })
                        .collect(),
                )
            }
        };

        // Parse the translation info field.
        let translation_info = match fields.get(25) {
            Some(&"") | None => None,
            Some(value) => {
                let translation_parts = <Self as GDELTObject>::delimited_vector(";", value);
                if let (Some(lang), Some(trans)) =
                    (translation_parts.get(0), translation_parts.get(1))
                {
                    Some((lang.to_string(), trans.to_string()))
                } else {
                    None
                }
            }
        };

        // If all fields are empty, return None. Otherwise, return a new GlobalKnowledgeGraph instance.
        if record_id.is_none()
            && v2_1_date.is_none()
            && source_collection_identifier.is_none()
            && source_common_name.is_none()
            && document_identifier.is_none()
            && counts_v1.is_none()
            && counts_v2.is_none()
            && themes_v1.is_none()
            && themes_v2.is_none()
            && locations_v1.is_none()
            && locations_v2.is_none()
            && persons_v1.is_none()
            && persons_v2.is_none()
            && organisations_v1.is_none()
            && organisations_v2.is_none()
            && tone.is_none()
            && enhanced_dates.is_none()
            && sharing_image.is_none()
            && related_images.is_none()
            && social_image_embeds.is_none()
            && social_video_embeds.is_none()
            && quotations.is_none()
            && all_names.is_none()
            && amounts.is_none()
            && translation_info.is_none()
        {
            None
        } else {
            Some(Self {
                record_id,
                v2_1_date,
                source_collection_identifier,
                source_common_name,
                document_identifier,
                counts: (counts_v1, counts_v2),
                themes: (themes_v1, themes_v2),
                locations: (locations_v1, locations_v2),
                persons: (persons_v1, persons_v2),
                organisations: (organisations_v1, organisations_v2),
                tone,
                enhanced_dates,
                sharing_image,
                related_images,
                social_image_embeds,
                social_video_embeds,
                quotations,
                all_names,
                amounts,
                translation_info,
            })
        }
    }
}