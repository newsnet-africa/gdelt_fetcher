#[cfg(test)]
mod query_types_test {
    use data::utils::types::api_types::{
        operation::Operation::{LessThan, MoreThan},
        query_types::QueryType,
        ToRequestLink,
    };
    use iso_country::Country;
    use models::models::gdelt::utils::gdelt_categorylist::GDELTCategoryList::LGBT;
    use models::models::gdelt::utils::gdelt_languages::GDELTLanguage;

    #[test]
    fn query_string_test() {
        let query = QueryType::QueryString("Nelson Mandela".to_string());
        assert_eq!(query.to_request_link(), "Nelson Mandela".to_string());
    }

    #[test]
    fn exclude_test() {
        let query1 = Box::new(QueryType::QueryString("Nelson Mandela".to_string()));
        let exclude = QueryType::Exclude(query1);
        assert_eq!(exclude.to_request_link(), "-\"Nelson Mandela\"".to_string());
    }

    #[test]
    fn domain_test() {
        let query = QueryType::Domain("example.com".to_string());
        assert_eq!(query.to_request_link(), "domain:example.com".to_string());
    }

    #[test]
    fn domain_is_test() {
        let query = QueryType::DomainIs("example.com".to_string());
        assert_eq!(query.to_request_link(), "domainis:example.com".to_string());
    }

    #[test]
    fn image_face_tone_test() {
        let query = QueryType::ImageFaceTone(MoreThan(0.5));
        assert_eq!(query.to_request_link(), "imagefacetone:>0.5".to_string());
    }

    #[test]
    fn image_num_faces_test() {
        let query = QueryType::ImageNumFaces(MoreThan(2));
        assert_eq!(query.to_request_link(), "imagenumfaces:>2".to_string());
    }

    #[test]
    fn image_orc_meta_test() {
        let query = QueryType::ImageORCMeta("meta".to_string());
        assert_eq!(query.to_request_link(), "imageorcmeta:\"meta\"".to_string());
    }

    #[test]
    fn image_tags_test() {
        let query = QueryType::ImageTags("tag".to_string());
        assert_eq!(query.to_request_link(), "imagetags:\"tag\"".to_string());
    }

    #[test]
    fn image_web_count_test() {
        let query = QueryType::ImageWebCount(MoreThan(100));
        assert_eq!(query.to_request_link(), "imagewebcount:>100".to_string());
    }

    #[test]
    fn image_web_tag_test() {
        let query = QueryType::ImageWebTag("tag".to_string());
        assert_eq!(query.to_request_link(), "imagewebtag:\"tag\"".to_string());
    }

    #[test]
    fn near_test() {
        let query = QueryType::Near {
            distance: 5,
            root_word: "word".to_string(),
        };
        assert_eq!(query.to_request_link(), "nearword:\"5\"".to_string());
    }

    #[test]
    fn repeat_test() {
        let query = QueryType::Repeat {
            distance: 5,
            root_word: "word".to_string(),
        };
        assert_eq!(query.to_request_link(), "repeatword:\"5\"".to_string());
    }

    #[test]
    fn source_country_test() {
        let query = QueryType::SourceCountry(Country::ZA);
        assert_eq!(
            query.to_request_link(),
            "sourcecountry:southafrica".to_string()
        );
    }

    #[test]
    fn source_lang_test() {
        let query = QueryType::SourceLang(GDELTLanguage::Thai);
        assert_eq!(query.to_request_link(), "sourcelang:tha".to_string());
    }

    #[test]
    fn theme_test() {
        let query = QueryType::Theme(LGBT);
        assert_eq!(query.to_request_link(), "theme:LGBT".to_string());
    }

    #[test]
    fn tone_test() {
        let query = QueryType::Tone(MoreThan(0.5));
        assert_eq!(query.to_request_link(), "tone:>0.5".to_string());
    }

    #[test]
    fn tone_abs_test() {
        let query = QueryType::ToneAbs(MoreThan(0.5));
        assert_eq!(query.to_request_link(), "toneabs:>0.5".to_string());
    }
}
