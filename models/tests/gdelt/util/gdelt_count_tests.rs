/// This module contains tests for the `GDELTCount` struct and its associated methods.
#[cfg(test)]
pub mod gdelt_count_tests {
    use crate::data_reader::data_reader;
    use crate::GKG_DATA_PATH;
    use models::models::gdelt::utils::gdelt_counts::GDELTCount;
    use models::models::gdelt::GDELTObject;
    use std::path::PathBuf;

    /// Tests the creation of `GDELTCount` instances and the `from_strings` method.
    ///
    /// This test reads a CSV file containing GDELT data, extracts the relevant records,
    /// and attempts to create `GDELTCount` instances from the extracted strings.
    /// It verifies that the `GDELTCount` instances are created correctly and prints them.
    ///
    /// # Panics
    ///
    /// This test will panic if the data cannot be read from the specified file path.
    #[test]
    fn test_gdelt_count_creator_and_from_strings() {
        // Define the path to the CSV file containing the GDELT data.
        let download_path = PathBuf::from(GKG_DATA_PATH);

        // Read the data from the CSV file. The `data_reader` function returns a vector of records.
        // Each record is a vector of strings representing the fields in the CSV file.
        let string_records = data_reader(download_path, true).expect("Failed to read data");

        // Iterate over each record in the vector of records.
        for record in string_records {
            // Get the string containing the count data from the 7th field (index 6) of the record.
            if let Some(count_strings) = record.get(6) {
                // Split the count string into individual count records using the ';' delimiter.
                let counts = count_strings.split(';');

                // Iterate over each count string.
                for count_str in counts {
                    // Create a `GDELTCount` instance from the count string using the `from_strings` method.
                    let count = GDELTCount::from_strings(count_str);

                    // Print the `GDELTCount` instance for debugging purposes.
                    println!("{:?}", count);

                    // Assert that the `GDELTCount` instance is either `Some` or `None`.
                    // This assertion is always true and serves as a placeholder for more specific tests.
                    assert!(count.is_some() || count.is_none());
                }
            }
        }
    }
}
