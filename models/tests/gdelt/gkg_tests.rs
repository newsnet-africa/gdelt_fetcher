#[cfg(test)]
pub mod gkg_test {
    use crate::data_reader::data_reader;
    use crate::{EXPORT_DATA_PATH, GKG_DATA_PATH};
    use models::models::gdelt::gkg::GlobalKnowledgeGraph;
    use models::models::gdelt::{GDELTObject, ToProto};
    use std::path::PathBuf;

    #[test]
    fn test_gkg_creator() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(GKG_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let k = record.clone().as_slice();

            let strings = record.iter().collect::<Vec<&str>>();

            let gkg = GlobalKnowledgeGraph::new(strings);

            println!("{:?}", gkg);

            assert!(gkg.is_some() || gkg.is_none())
        }
    }
    
    #[test]
    fn to_proto() {
        let download_path = PathBuf::from(GKG_DATA_PATH);

        let string_records = data_reader(download_path, true).expect("Failed to read data");

        for record in string_records {
            let strings = record.iter().collect::<Vec<&str>>();

            let gkg = GlobalKnowledgeGraph::new(strings).unwrap();

            let proto = gkg.to_proto();

            assert!(proto.is_some() || proto.is_none())
        }
    }
}
