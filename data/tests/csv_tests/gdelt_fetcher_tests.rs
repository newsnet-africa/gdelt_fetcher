#[cfg(test)]
pub mod gdelt_tests {
    // region Imports
    use data::gdelt_fetcher::{FetchType, GdeltFileFetcher};
    // endregion

    /// Tests the `fetch` function of `GdeltFileFetcher`.
    ///
    /// This test creates a new `GdeltFileFetcher` instance and attempts to fetch data from a specified URL.
    /// It asserts that the result is `Ok`.
    #[tokio::test]
    async fn test_fetch() {
        // region Setup
        let mut fetcher = GdeltFileFetcher::new();
        let url = "http:data.gdeltproject.org/gdeltv2/20240515224500.export.CSV.zip";
        // endregion

        // region Execute
        let result = fetcher.fetch(url).await;
        // endregion

        // region Assert
        assert!(result.is_ok());
        // endregion
    }

    /// Tests the `fetch_events` function of `GdeltFileFetcher`.
    ///
    /// This test creates a new `GdeltFileFetcher` instance and attempts to fetch event data for the current date.
    /// It asserts that the result is `Ok`.
    #[tokio::test]
    async fn test_fetch_events() {
        // region Setup
        let mut fetcher = GdeltFileFetcher::new();
        let date = GdeltFileFetcher::last_update_string();
        // endregion

        // region Execute
        let result = fetcher.fetch_events(&date).await;
        // endregion

        // region Assert
        assert!(result.is_ok());
        // endregion
    }

    /// Tests the `fetch_mentions` function of `GdeltFileFetcher`.
    ///
    /// This test creates a new `GdeltFileFetcher` instance and attempts to fetch mention data for the current date.
    /// It asserts that the result is `Ok`.
    #[tokio::test]
    async fn test_fetch_mentions() {
        // region Setup
        let mut fetcher = GdeltFileFetcher::new();
        let date = GdeltFileFetcher::last_update_string();
        // endregion

        // region Execute
        let result = fetcher.fetch_mentions(&date).await;
        // endregion

        // region Assert
        assert!(result.is_ok());
        // endregion
    }

    /// Tests the `fetch_gkg` function of `GdeltFileFetcher`.
    ///
    /// This test creates a new `GdeltFileFetcher` instance and attempts to fetch GKG data for the current date.
    /// It asserts that the result is `Ok`.
    #[tokio::test]
    async fn test_fetch_gkg() {
        // region Setup
        let mut fetcher = GdeltFileFetcher::new();
        let date = GdeltFileFetcher::last_update_string();
        // endregion

        // region Execute
        let result = fetcher.fetch_gkg(&date).await;
        // endregion

        // region Assert
        assert!(result.is_ok());
        // endregion
    }

    /// Tests the `fetch_type` function of `GdeltFileFetcher`.
    ///
    /// This test creates a new `GdeltFileFetcher` instance and attempts to fetch event data for the current date using the `fetch_type` function.
    /// It asserts that the result is `Ok`.
    #[tokio::test]
    async fn test_fetch_type() {
        // region Setup
        let mut fetcher = GdeltFileFetcher::new();
        let date = GdeltFileFetcher::last_update_string();
        // endregion

        // region Execute
        // Iterate over all FetchType variants
        for fetch_type in [FetchType::EVENTS, FetchType::MENTIONS, FetchType::GKG] {
            let fetch_type_clone = fetch_type.clone(); // Dereference fetch_type to get the value
            let result = fetcher.fetch_type(fetch_type_clone, &date).await;
            // region Assert
            assert!(result.is_ok(), "Failed to fetch data for {:?}", fetch_type);
            // endregion
        }
    }

    /// Tests the `fetch_latest` function of `GdeltFileFetcher`.
    ///
    /// This test creates a new `GdeltFileFetcher` instance and attempts to fetch the latest data for all types (EVENTS, MENTIONS, GKG).
    /// It asserts that the result is `Ok`.
    #[tokio::test]
    async fn test_fetch_latest() {
        // region Setup
        let mut fetcher = GdeltFileFetcher::new();
        // endregion

        // region Execute
        // Iterate over all FetchType variants
        for fetch_type in [FetchType::EVENTS, FetchType::MENTIONS, FetchType::GKG] {
            let result = fetcher.fetch_latest(fetch_type.clone()).await;
            // region Assert
            assert!(
                result.is_ok(),
                "Failed to fetch latest data for {:?}",
                fetch_type
            );
            // endregion
        }
    }
}
