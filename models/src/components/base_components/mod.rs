pub mod actor;
pub mod event;
pub mod global_difference_graph;
pub mod global_knowledge_graph;
pub mod location;
pub mod mention;

pub mod raw_types {
    use count::{RawCountObject, RawCountValue};
    #[repr(transparent)]
    pub struct RawGlobalEventID(u128);
    #[repr(transparent)]
    pub struct RawDay(u32);
    #[repr(transparent)]
    pub struct RawMonthYear(u32);
    #[repr(transparent)]
    pub struct RawYear(u16);
    #[repr(transparent)]
    pub struct RawFractionDate(f64);

    #[repr(transparent)]
    pub struct RawCAMEOActorCode([[u8; 3]; 5]);
    #[repr(transparent)]
    pub struct RawActorName<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawCAMEOCountryCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct RawCAMEOKnownGroupCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct RawCAMEOEthnicCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct RawCAMEOReligionCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct RawCAMEOActorTypeCode(pub [u8; 3]);

    #[repr(transparent)]
    pub struct RawIsRootEvent(bool);
    #[repr(transparent)]
    pub struct RawCAMEOEventCode(pub [u8; 4]);
    #[repr(transparent)]
    pub struct RawCAMEOEventBaseCode([u8; 3]);
    #[repr(transparent)]
    pub struct RawCAMEOEventRootCode([u8; 2]);
    #[repr(transparent)]
    pub struct RawQuadClass(pub u8);
    #[repr(transparent)]
    pub struct RawGoldsteinScale(pub f32);
    #[repr(transparent)]
    pub struct RawNumberOfMentions(pub u128);
    #[repr(transparent)]
    pub struct RawNumberOfSources(pub u128);
    #[repr(transparent)]
    pub struct RawNumberOfArticles(pub u128);
    #[repr(transparent)]
    pub struct RawAverageTone(pub f32);
    #[repr(transparent)]
    pub struct RawDateAdded(pub u64);

    pub enum RawSourceUrl<'a> {
        URL(&'a str),
        Citation(&'a str),
    }
    #[repr(transparent)]
    pub struct RawSentenceID(u128);
    #[repr(transparent)]
    pub struct RawInRawText(bool);
    #[repr(transparent)]
    pub struct RawConfidence(f32);
    #[repr(transparent)]
    pub struct RawDocLength(u128);

    #[repr(transparent)]
    pub struct RawGlobalKnowledgeGraphRecordID<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawV2Date(u64);
    #[repr(transparent)]
    pub struct RawV2SourceCollectionIdentifier(u8);
    #[repr(transparent)]
    pub struct RawV2SourceCommonName<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawV2DocumentIdentifier<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawCharOffset(u128);
    #[repr(transparent)]
    pub struct RawV1Theme<'a>(&'a str);
    pub struct RawV2Theme<'a>(&'a str, RawCharOffset);
    #[repr(transparent)]
    pub struct RawV1Person<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawV2Person<'a>(&'a RawCharOffset);
    #[repr(transparent)]
    pub struct RawV1Organisation<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawV2Organisation<'a>(&'a RawCharOffset);
    #[repr(transparent)]
    pub struct RawV2SharingImage<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawV2RelatedImage<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawV2SocialMediaEmbed<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawV2SocialVideoEmbed<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawV2AllName<'a>(&'a str);
    pub struct RawV2Amount<'a>(RawCountValue, RawCountObject<'a>, RawCharOffset);

    #[repr(transparent)]
    pub struct RawPageURL<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawPageTitle<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawPageDomainFull<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawPageDomainRoot<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawPageLanguage<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawFetchDateOriginal<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawFetchDateCheck<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawHTTPCode<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawHTTPSize(u128);
    #[repr(transparent)]
    pub struct RawRedirectURL<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawTitleNew<'a>(&'a str);
    #[repr(transparent)]
    pub struct RawNumberOfChanges(u128);
    pub enum RawChangeUnit {
        Word,
        Char,
    }
    #[repr(transparent)]
    pub struct RawFromNumberOfChars(u128);
    #[repr(transparent)]
    pub struct RawToNumberOfChars(u128);
    #[repr(transparent)]
    pub struct RawFromNumberOfChangedChars(u128);
    #[repr(transparent)]
    pub struct RawToNumberOfChangedChars(u128);
    #[repr(transparent)]
    pub struct RawTotalChangedChars(u128);
    #[repr(transparent)]
    pub struct RawPercentChangedChars(u128);
    pub mod changes {
        pub struct RawChanges<'a>(
            RawOriginalTextBlock<'a>,
            RawNewTextBlock<'a>,
            RawFromRange,
            RawToRange,
        );
        #[repr(transparent)]
        pub struct RawOriginalTextBlock<'a>(&'a str);
        #[repr(transparent)]
        pub struct RawNewTextBlock<'a>(&'a str);
        pub struct RawFromRange(u128, u128);
        pub struct RawToRange(u128, u128);
    }

    pub mod extras_xml {
        pub mod cited_reference {
            pub struct RawCitedReference<'a>(
                pub RawAuthor<'a>,
                pub RawTitle<'a>,
                pub RawBookTitle<'a>,
                pub RawDate<'a>,
                pub RawJournal<'a>,
                pub RawVolume<'a>,
                pub RawIssue<'a>,
                pub RawPages<'a>,
                pub RawInstitution<'a>,
                pub RawPublisher<'a>,
                pub RawLocation<'a>,
            );
            #[repr(transparent)]
            pub struct RawAuthor<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawTitle<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawBookTitle<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawDate<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawJournal<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawVolume<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawIssue<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawPages<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawInstitution<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawPublisher<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawLocation<'a>(&'a str);
            #[repr(transparent)]
            pub struct RawMarker<'a>(&'a str);
        }
    }
    pub mod language {
        pub struct RawTranslationInfo<'a>(
            RawSourceLanguageCode,
            (RawTranslationEngine<'a>, RawTranslationModel<'a>),
        );
        #[repr(transparent)]
        pub struct RawSourceLanguageCode([u8; 3]);
        #[repr(transparent)]
        pub struct RawTranslationEngine<'a>(&'a str);
        #[repr(transparent)]
        pub struct RawTranslationModel<'a>(&'a str);
    }
    pub mod quotation {
        use super::RawCharOffset;

        pub struct RawV2Quotation<'a>(
            RawCharOffset,
            RawQuotationLength,
            RawQuotationVerb<'a>,
            RawQuoteValue<'a>,
        );
        #[repr(transparent)]
        pub struct RawQuotationLength(u128);
        #[repr(transparent)]
        pub struct RawQuotationVerb<'a>(&'a str);
        #[repr(transparent)]
        pub struct RawQuoteValue<'a>(&'a str);
    }
    pub mod date {
        pub struct RawV2EnhancedDate(RawDateResolution, RawMonth, RawDay, RawYear);
        #[repr(transparent)]
        pub struct RawDateResolution(u8);
        #[repr(transparent)]
        pub struct RawMonth(u8);
        #[repr(transparent)]
        pub struct RawDay(u8);
        #[repr(transparent)]
        pub struct RawYear(u16);
    }

    pub mod tone {
        pub struct RawV1Tone(
            pub RawToneValue,
            pub RawPositiveScore,
            pub RawNegativeScore,
            pub RawPolarity,
            pub RawActivityReferenceDensity,
            pub RawPronounReferenceDensity,
            pub RawWordCount,
        );
        #[repr(transparent)]
        pub struct RawToneValue(f32); //TODO: Maybe use a crate that shrinks these values cause the range is only from -10 to +10. Do this for all other f32s in this crate I guess
        #[repr(transparent)]
        pub struct RawPositiveScore(f32);
        #[repr(transparent)]
        pub struct RawNegativeScore(f32);
        #[repr(transparent)]
        pub struct RawPolarity(f32);
        #[repr(transparent)]
        pub struct RawActivityReferenceDensity(f32);
        #[repr(transparent)]
        pub struct RawPronounReferenceDensity(f32);
        #[repr(transparent)]
        pub struct RawWordCount(u128);
    }

    pub mod location {
        use super::RawCharOffset;

        pub struct RawV1Location<'a>(
            pub RawLocationType,
            pub RawLocationName<'a>,
            pub RawFIPSCountryCode,
            pub RawFIPSAdministrationCode,
            pub RawLatitude,
            pub RawLongitude,
            pub RawFeatureID<'a>,
        );
        pub struct RawV2Location<'a>(
            pub RawLocationType,
            pub RawLocationName<'a>,
            pub RawFIPSCountryCode,
            pub RawFIPSAdministrationCode,
            pub RawLatitude,
            pub RawLongitude,
            pub RawFeatureID<'a>,
            pub RawCharOffset,
        );
        #[repr(transparent)]
        pub struct RawLocationType(pub u8);
        #[repr(transparent)]
        pub struct RawLocationName<'a>(pub &'a str);
        #[repr(transparent)]
        pub struct RawFIPSCountryCode(pub [u8; 2]);
        pub struct RawFIPSAdministrationCode(pub RawFIPSCountryCode, [u8; 2]);
        #[repr(transparent)]
        pub struct RawLatitude(pub f64);
        #[repr(transparent)]
        pub struct RawLongitude(pub f64);
        pub enum RawFeatureID<'a> {
            String(&'a str),
            Integer(i128),
        }
    }
    pub mod count {
        use super::{
            RawCharOffset,
            location::{
                RawFIPSAdministrationCode, RawFIPSCountryCode, RawFeatureID, RawLatitude,
                RawLocationName, RawLocationType, RawLongitude,
            },
        };

        pub struct RawV1Count<'a>(
            RawCountType<'a>,
            RawCountValue,
            RawCountObject<'a>,
            RawLocationType,
            RawLocationName<'a>,
            RawFIPSCountryCode,
            RawFIPSAdministrationCode,
            RawLongitude,
            RawLatitude,
            RawFeatureID<'a>,
        );
        pub struct RawV2Count<'a>(
            RawCountType<'a>,
            RawCountValue,
            RawCountObject<'a>,
            RawLocationType,
            RawLocationName<'a>,
            RawFIPSCountryCode,
            RawFIPSAdministrationCode,
            RawLongitude,
            RawLatitude,
            RawFeatureID<'a>,
            RawCharOffset,
        );
        #[repr(transparent)]
        pub struct RawCountType<'a>(&'a str);
        #[repr(transparent)]
        pub struct RawCountValue(u128);
        #[repr(transparent)]
        pub struct RawCountObject<'a>(&'a str);
    }
}
