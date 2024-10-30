#[cfg(test)]
pub mod events_test {
    use crate::data_reader::data_reader;
    use crate::EXPORT_DATA_PATH;
    use models::models::gdelt::event::Event;
    use models::models::gdelt::{GDELTObject, ToProto};
    use std::path::PathBuf;

    #[test]
    fn test_event_creator() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let strings = record.iter().collect::<Vec<&str>>();

            let event = Event::new(strings);

            println!("{:?}", event);

            assert!(event.is_some() || event.is_none())
        }
    }
    
    #[test]
    fn test_to_proto() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let strings = record.iter().collect::<Vec<&str>>();

            let event = Event::new(strings).unwrap();

            let proto = event.to_proto();

            println!("{:?}", proto);

            assert!(proto.is_some() || proto.is_none())
        }
    }
}
