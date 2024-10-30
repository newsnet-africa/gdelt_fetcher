/// This module contains tests for the `csv_to_struct` function.
#[cfg(test)]
mod csv_to_struct_tests {
    // region Imports
    
    use data::csv_to_structs::{csv_to_structs, CsvToStructError};
    use data::gdelt_fetcher::{FetchType, GdeltFileFetcher};
    use data::utils::types::DatabaseType;
    use models::models::gdelt::DatabaseTableEnum;
    // endregion

    /// This test function verifies the `csv_to_struct` function.
    ///
    /// It performs the following steps:
    /// 1. Defines an asynchronous block to perform the test operations.
    /// 2. Fetches the latest data for EVENTS, MENTIONS, and GKG.
    /// 3. Converts the fetched CSV data to structs for different database types.
    /// 4. Asserts that the conversion result is successful.
    ///
    /// # Panics
    /// This test will panic if fetching the data or converting the CSV to structs fails.
    #[tokio::test]
    async fn test_csv_to_struct() {
        // region Define Asynchronous Block
        let result: Result<Vec<DatabaseTableEnum>, CsvToStructError> = async {
            // region Define Date String
            // Define the date string for fetching data
            let date = GdeltFileFetcher::last_update_string();
            // endregion

            // region Initialize Fetcher
            // Initialize the GdeltFileFetcher instance
            let mut fetcher = GdeltFileFetcher::new();
            // endregion

            // region Fetch Data
            // Fetch the latest EVENTS data
            let event_result = fetcher
                .fetch_latest(FetchType::EVENTS)
                .await
                .expect("Can't fetch Events");

            // Fetch the latest MENTIONS data
            let mentions_result = fetcher
                .fetch_latest(FetchType::MENTIONS)
                .await
                .expect("Can't fetch mentions");

            // Fetch the latest GKG data
            let gkg_result = fetcher
                .fetch_latest(FetchType::GKG)
                .await
                .expect("Can't fetch GKG");
            // endregion

            // region Convert CSV to Struct
            // Convert the fetched CSV data to a struct for the Export database type
            let output_export = csv_to_structs(&date, DatabaseType::Export).await?;

            // Convert the fetched CSV data to a struct for the Mentions database type
            let output_mentions = csv_to_structs(&date, DatabaseType::Mentions).await?;

            // Convert the fetched CSV data to a struct for the GKG database type
            let output_gkg = csv_to_structs(&date, DatabaseType::GKG).await?;
            // endregion

            // Return the output as a Result
            Ok(output_export)
        }
            .await;
        // endregion

        // region Assert Result
        // Assert that the result is successful
        assert!(result.is_ok());
        // endregion
    }
}
