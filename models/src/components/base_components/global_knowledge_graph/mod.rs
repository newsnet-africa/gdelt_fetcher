use crate::schema::mention_table::CharOffset;

use super::raw_types::{
    RawGlobalKnowledgeGraphRecordID, RawV1Organisation, RawV1Person, RawV1Theme, RawV2AllName,
    RawV2Amount, RawV2Date, RawV2DocumentIdentifier, RawV2Organisation, RawV2Person,
    RawV2RelatedImage, RawV2SharingImage, RawV2SocialMediaEmbed, RawV2SocialVideoEmbed,
    RawV2SourceCollectionIdentifier, RawV2SourceCommonName, RawV2Theme,
    count::{RawCountValue, RawV1Count, RawV2Count},
    date::RawV2EnhancedDate,
    extras_xml::cited_reference::{RawCitedReference, RawDate},
    language::RawTranslationInfo,
    location::{RawV1Location, RawV2Location},
    quotation::RawV2Quotation,
    tone::RawV1Tone,
};

pub mod gdelt_category;

pub struct RawGlobalKnowledgeGraph<'a> {
    gkg_record: RawGlobalKnowledgeGraphRecordID<'a>,
    v2_date: RawDate<'a>,
    v2_source_collection_identifier: RawV2SourceCollectionIdentifier,
    v2_source_common_name: RawV2SourceCommonName<'a>,
    v2_document_identifier: RawV2DocumentIdentifier<'a>,
    v1_counts: RawV1Count<'a>,
    v2_counts: RawV2Count<'a>,
    v1_themes: RawV1Theme<'a>,
    v2_themes: RawV2Theme<'a>,
    v1_locations: RawV1Location<'a>,
    v2_locations: RawV2Location<'a>,
    v1_persons: RawV1Person<'a>,
    v2_persons: RawV2Person<'a>,
    v1_organisations: RawV1Organisation<'a>,
    v2_organisations: RawV2Organisation<'a>,
    v15_tone: RawV1Tone,
    v2_dates: RawV2EnhancedDate,
    v2_sharing_image: RawV2SharingImage<'a>,
    v2_related_images: RawV2RelatedImage<'a>,
    v2_social_media_embeds: RawV2SocialMediaEmbed<'a>,
    v2_social_video_embeds: RawV2SocialVideoEmbed<'a>,
    v2_quotations: RawV2Quotation<'a>,
    v2_all_names: RawV2AllName<'a>,
    v2_amounts: RawV2Amount<'a>,
    v2_translation_info: RawTranslationInfo<'a>,
    v2_extra_xml: RawCitedReference<'a>,
}
