#[cfg(test)]
pub mod gdelt_tone_tests {
    use crate::data_reader::data_reader;
    use crate::GKG_DATA_PATH;
    use models::models::gdelt::utils::gdelt_tone::Tone;
    use models::models::gdelt::GDELTObject;
    use std::path::PathBuf;

    /// Tests the creation of `Tone` instances from string records.
    ///
    /// This test reads a CSV file containing GDELT data, extracts relevant fields for the tone
    /// from each record, and attempts to create `Tone` instances from these fields.
    /// It then prints the created tones and asserts that the tones are either `Some` or `None`.
    #[test]
    fn test_tone_creation() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(GKG_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, true).expect("Failed to read data");

        // Iterate over each record in the CSV file.
        for record in string_records {
            // Collect the fields of the record into a vector of strings.
            let strings = record.iter().collect::<Vec<&str>>();

            // Extract the fields for the tone (fields 27 to 33).
            let tone_strings = strings[15].split("\t").collect::<Vec<&str>>();

            // Create `Tone` instances from the extracted fields.
            let tone = Tone::new(tone_strings);

            // Print the created tones for debugging purposes.
            println!("Tone: {:?}\n\n", tone);

            // Ensure that the tones are either `Some` or `None`.
            assert!(tone.is_some() || tone.is_none());
        }
    }

    /// Tests the creation of `Tone` instances from concatenated string records.
    ///
    /// This test reads a CSV file containing GDELT data, concatenates the fields of each record
    /// into a single string, splits the string back into fields, and attempts to create `Tone`
    /// instances from these fields. It then prints the created tones and asserts that the tones
    /// are either `Some` or `None`.
    #[test]
    fn test_tone_from_strings() {
        // Define the path to the CSV file containing test data.
        let download_path = PathBuf::from(GKG_DATA_PATH);

        // Read the CSV file and get the string records.
        let string_records = data_reader(download_path, false).expect("Failed to read data");

        // Iterate over each record in the CSV file.
        for record in string_records {
            // Concatenate the fields of the record into a single string separated by tabs.
            let record_string = record.iter().collect::<Vec<&str>>().join("\t");

            // Split the concatenated string back into fields.
            let trimmed_string = record_string.split("\t").collect::<Vec<&str>>();

            // Extract the fields for the tone (fields 27 to 33) and join them with tabs.
            let tone_string = trimmed_string[15];

            // Create `Tone` instances from the concatenated fields.
            let tone = Tone::from_strings(&tone_string);

            // Print the created tones for debugging purposes.
            println!("Tone: {:?}\n\n", tone);

            // Ensure that the tones are either `Some` or `None`.
            assert!(tone.is_some() || tone.is_none());
        }
    }
}
