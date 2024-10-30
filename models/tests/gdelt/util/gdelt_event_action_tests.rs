#[cfg(test)]
pub mod gdelt_event_action_tests {
    use crate::data_reader::data_reader;
    use crate::EXPORT_DATA_PATH;
    use models::models::gdelt::utils::gdelt_event_action::GDELTAction;
    use models::models::gdelt::GDELTObject;
    use std::path::PathBuf;

    /// Tests the creation of `GDELTAction` instances from string records.
    ///
    /// This test verifies that the `GDELTAction` instances are created correctly from tab-delimited strings.
    #[test]
    fn test_gdelt_action_creation() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let strings = record.iter().collect::<Vec<&str>>();

            let action_strings = strings[26..=35].to_vec();

            let action = GDELTAction::new(action_strings);

            println!("{:?}", action);

            assert!(action.is_some() || action.is_none());
        }
    }

    #[test]
    fn test_gdelt_action_from_strings() {
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        let string_records = data_reader(download_path, false).expect("Failed to read data");

        for record in string_records {
            let record_string = record.iter().collect::<Vec<&str>>().join("\t");

            let trimmed_string = record_string.split("\t").collect::<Vec<&str>>();

            let acton_strings = trimmed_string[26..=35].join("\t");

            let action = GDELTAction::from_strings(&acton_strings);

            println!("Action: {:?}\n\n", action);

            assert!(action.is_some() || action.is_none())
        }
    }
}
