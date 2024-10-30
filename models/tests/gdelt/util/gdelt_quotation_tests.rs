#[cfg(test)]
pub mod gdelt_quotation_tests {
    use crate::data_reader::data_reader;
    use crate::GKG_DATA_PATH;
    use models::models::gdelt::utils::gdelt_quotation::GDELTQuotation;
    use models::models::gdelt::GDELTObject;
    use std::path::PathBuf;

    #[test]
    fn test_from_strings() {
        let download_path = PathBuf::from(GKG_DATA_PATH);

        let string_records = data_reader(download_path, true).expect("Failed to read Data");

        for record in string_records {
            let record_string = record.iter().collect::<Vec<&str>>();

            let quotation_string = record_string.get(22);

            let quotation = match quotation_string {
                Some(&"") | None => None,
                Some(value) => GDELTQuotation::from_strings(value),
            };

            assert!(quotation.is_some() || quotation.is_none())
        }
    }
}
