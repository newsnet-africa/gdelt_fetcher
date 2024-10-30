#[cfg(test)]
pub mod mentions_test {
    use crate::data_reader::data_reader;
    use crate::MENTION_DATA_PATH;
    use models::models::gdelt::gkg::GlobalKnowledgeGraph;
    use models::models::gdelt::mentions::Mentions;
    use models::models::gdelt::{GDELTObject, ToProto};
    use std::path::PathBuf;

    #[test]
    fn test_mention_creator() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(MENTION_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let strings = record.iter().collect::<Vec<&str>>();

            let mention = Mentions::new(strings);

            println!("{:?}", mention);

            assert!(mention.is_some() || mention.is_none())
        }
    }
    
    #[test]
    fn test_to_proto() {
        let download_path = PathBuf::from(MENTION_DATA_PATH);

        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let strings = record.iter().collect::<Vec<&str>>();

            let mention = Mentions::new(strings).unwrap();

            let proto = mention.to_proto();

            assert!(proto.is_some() || proto.is_none())
        }
    }
}
