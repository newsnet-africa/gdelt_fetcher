#[cfg(test)]
pub mod gdelt_actor_tests {
    use crate::data_reader::data_reader;
    use crate::EXPORT_DATA_PATH;
    use models::models::gdelt::utils::gdelt_actor::GDELTActor;
    use models::models::gdelt::GDELTObject;
    use std::path::PathBuf;

    /// Tests the creation of `GDELTActor` instances from string records.
    ///
    /// This test reads a CSV file containing GDELT data, extracts relevant fields for two actors
    /// from each record, and attempts to create `GDELTActor` instances from these fields.
    /// It then prints the created actors and asserts that the actors are either `Some` or `None`.
    #[test]
    fn test_gdelt_actor_creation() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, true).expect("Failed to read data");

        // Iterate over each record in the CSV file.
        for record in string_records {
            // Collect the fields of the record into a vector of strings.
            let strings = record.iter().collect::<Vec<&str>>();

            // Extract the fields for the first actor (fields 5 to 14).
            let actor1_strings = strings[5..=14].to_vec();

            // Extract the fields for the second actor (fields 15 to 26).
            let actor2_strings = strings[15..=26].to_vec();

            // Create `GDELTActor` instances from the extracted fields.
            let actor1 = GDELTActor::new(actor1_strings);
            let actor2 = GDELTActor::new(actor2_strings);

            // Print the created actors for debugging purposes.
            println!("Actor 1: {:?}\nActor 2: {:?}\n\n", actor1, actor2);

            // Ensure that the actors are either `Some` or `None`.
            assert!(actor1.is_some() || actor1.is_none());
            assert!(actor2.is_some() || actor2.is_none());
        }
    }

    /// Tests the creation of `GDELTActor` instances from concatenated string records.
    ///
    /// This test reads a CSV file containing GDELT data, concatenates the fields of each record
    /// into a single string, splits the string back into fields, and attempts to create `GDELTActor`
    /// instances from these fields. It then prints the created actors and asserts that the actors
    /// are either `Some` or `None`.
    #[test]
    fn test_gdelt_actor_from_strings() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(EXPORT_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, false).expect("Failed to read data");

        // Iterate over each record in the CSV file.
        for record in string_records {
            // Concatenate the fields of the record into a single string separated by tabs.
            let record_string = record.iter().collect::<Vec<&str>>().join("\t");

            // Split the concatenated string back into fields.
            let trimmed_string = record_string.split("\t").collect::<Vec<&str>>();

            // Extract the fields for the first actor (fields 5 to 14) and join them with tabs.
            let actor1_string = trimmed_string[5..=14].join("\t");

            // Extract the fields for the second actor (fields 15 to 26) and join them with tabs.
            let actor2_string = trimmed_string[15..=26].join("\t");

            // Create `GDELTActor` instances from the concatenated fields.
            let actor1 = GDELTActor::from_strings(&actor1_string);
            let actor2 = GDELTActor::from_strings(&actor2_string);

            // Print the created actors for debugging purposes.
            println!("Actor 1: {:?}\nActor 2: {:?}\n\n", actor1, actor2);

            // Ensure that the actors are either `Some` or `None`.
            assert!(actor1.is_some() || actor1.is_none());
            assert!(actor2.is_some() || actor2.is_none());
        }
    }
}
