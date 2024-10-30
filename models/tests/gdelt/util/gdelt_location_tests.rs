#[cfg(test)]
pub mod gdelt_location_tests {
    use crate::data_reader::data_reader;
    use crate::EXPORT_DATA_PATH;
    use models::models::gdelt::event::Event;
    use models::models::gdelt::utils::gdelt_location::GDELTLocation;
    use models::models::gdelt::{DatabaseTableEntry, DatabaseTableEnum, GDELTObject};
    use std::path::PathBuf;

    #[test]
    fn test_gdelt_location_from_strings() {
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        let string_records = data_reader(download_path, false).expect("Failed to read data");

        for record in string_records {
            let record_string = record.iter().collect::<Vec<&str>>().join("\t");

            let trimmed_string = record_string.split("\t").collect::<Vec<&str>>();

            let location_string = trimmed_string[35..=42].join("\t");

            let location = GDELTLocation::from_strings(&location_string);

            println!("Location: {:?}\n", location);

            assert!(location.is_some() || location.is_none())
        }
    }

    #[test]
    fn test_gdelt_location_creator() {
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let strings = record.iter().collect::<Vec<&str>>();

            let location_strings = strings[35..=42].to_vec();

            let location = GDELTLocation::new(location_strings);

            println!("Location: {:?}", location);

            assert!(location.is_some() || location.is_none())
        }
    }

    #[test]
    fn test_gdelt_from_table() {
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let record_string = record.iter().collect::<Vec<&str>>().join("\t");

            let trimmed_string = record_string.split("\t").collect::<Vec<&str>>();

            let location_string = trimmed_string[35..=42].join("\t");

            let location = GDELTLocation::from_table(
                &location_string,
                DatabaseTableEnum::Event(None),
            );

            println!("Location: {:?}\n", location);

            assert!(location.is_some() || location.is_none())
        }
    }
}
