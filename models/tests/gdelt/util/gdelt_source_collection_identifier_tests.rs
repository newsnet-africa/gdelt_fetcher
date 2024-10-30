#[cfg(test)]
pub mod gdelt_source_collection_identifier {
    use models::models::gdelt::utils::gdelt_source_collection_identifier::SourceCollectionIdentifier;

    #[test]
    fn test_from_u8_to_u8() {
        for i in 0..=6 {
            let sci = SourceCollectionIdentifier::from(i);
            let u_8 = u8::from(sci.clone());
            println!("{:?}", sci);
            assert_eq!(i, u_8)
        }
    }

    #[test]
    fn test_from_string() {
        for i in 0..6 {
            let i_string = i.to_string();

            let sci = SourceCollectionIdentifier::from_string(&i_string);

            println!("{:?}", sci);
            assert!(sci.is_some() || sci.is_none())
        }
    }
}
