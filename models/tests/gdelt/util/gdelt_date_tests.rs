/// This module contains tests for the `GDELTDate` struct and its associated methods.
#[cfg(test)]
pub mod gdelt_date_tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use models::models::gdelt::utils::gdelt_date::{DateResolution, GDELTDate};

    /// Tests the `check_resolution` method of the `GDELTDate` struct.
    ///
    /// This test verifies that the `check_resolution` method correctly identifies the resolution
    /// of various date representations. The test data includes different date formats and their
    /// expected resolutions.
    ///
    /// The `check_resolution` method is expected to return the appropriate `DateResolution` enum
    /// variant for valid date formats and `None` for invalid formats.
    #[test]
    fn test_date_check_resolution() {
        // Define test data as tuples of input date representations and their expected resolutions.
        let test_data = [
            (20240101, Some(DateResolution::YearMonthDate)), // YYYYMMDD format
            (202401, Some(DateResolution::YearMonth)),       // YYYYMM format
            (0101, None),                                    // Invalid format
            (2024, Some(DateResolution::Year)),              // YYYY format
            (
                2024_01_01_00_00_00,
                Some(DateResolution::YearMonthDayHourMinuteSecond),
            ), // YYYYMMDDHHMMSS format
        ];

        // Iterate over the test data and assert that the `check_resolution` method returns the expected resolution.
        for (input, expected) in test_data.iter() {
            assert_eq!(GDELTDate::check_resolution(*input), *expected);
        }
    }

    /// Tests the `date_from_int` method of the `GDELTDate` struct.
    ///
    /// This test verifies that the `date_from_int` method correctly converts various integer
    /// representations of dates into `NaiveDateTime` instances. The test data includes different
    /// date formats and their expected `NaiveDateTime` representations.
    ///
    /// The `date_from_int` method is expected to return the appropriate `NaiveDateTime` instance
    /// for valid date formats and `None` for invalid formats.
    #[test]
    fn test_date_from_int() {
        // Define test data as tuples of input date representations and their expected `NaiveDateTime` instances.
        let test_data = [
            (
                20240101,
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                )),
            ), // YYYYMMDD format
            (
                202401,
                Some(NaiveDateTime::from(
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                )),
            ), // YYYYMM format
            (0101, None), // Invalid format
            (
                2024,
                Some(NaiveDateTime::from(
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                )),
            ), // YYYY format
            (
                2024_01_01_00_00_00,
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                )),
            ), // YYYYMMDDHHMMSS format
        ];

        // Iterate over the test data and assert that the `date_from_int` method returns the expected `NaiveDateTime` instance.
        for (input, expected) in test_data.iter() {
            assert_eq!(GDELTDate::naive_date_from_int(*input), *expected);
        }
    }

    /// Tests the `date_from_string` and `from_string` methods of the `GDELTDate` struct.
    ///
    /// This test verifies that the `date_from_string` and `from_string` methods correctly convert
    /// various string representations of dates into `NaiveDateTime` instances. The test data includes
    /// different date formats and their expected `NaiveDateTime` representations.
    ///
    /// The `date_from_string` and `from_string` methods are expected to return the appropriate
    /// `NaiveDateTime` instance for valid date formats and `None` for invalid formats.
    #[test]
    fn test_date_from_string_and_from_string() {
        // Define test data as tuples of input date representations and their expected `NaiveDateTime` instances.
        let test_data = [
            (
                "20240101",
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                )),
            ), // YYYYMMDD format
            (
                "202401",
                Some(NaiveDateTime::from(
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                )),
            ), // YYYYMM format
            ("0101", None), // Invalid format
            (
                "2024",
                Some(NaiveDateTime::from(
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                )),
            ), // YYYY format
            (
                "20240101000000",
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                )),
            ), // YYYYMMDDHHMMSS format
        ];

        // Iterate over the test data and assert that the `date_from_string` method returns the expected `NaiveDateTime` instance.
        for (input, expected) in test_data.iter() {
            assert_eq!(GDELTDate::naive_date_from_string(*input), *expected);
        }
    }

    /// Tests the `date_from_float` method of the `GDELTDate` struct.
    ///
    /// This test verifies that the `date_from_float` method correctly converts a float representation
    /// of a date into a `NaiveDateTime` instance. The test data includes a float date and its expected
    /// `NaiveDateTime` representation.
    ///
    /// The `date_from_float` method is expected to return the appropriate `NaiveDateTime` instance
    /// for valid float date formats.
    #[test]
    fn test_date_from_float() {
        // Define the input float date and its expected `NaiveDateTime` instance.
        let float_input = 2024.6172;
        let expected = Some(NaiveDateTime::from(
            NaiveDate::from_ymd_opt(2024, 8, 12).unwrap(),
        ));

        // Assert that the `date_from_float` method returns the expected `NaiveDateTime` instance.
        assert_eq!(GDELTDate::naive_date_from_float(float_input), expected);
    }
}
