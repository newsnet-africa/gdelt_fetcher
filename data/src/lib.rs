/// The `core` crate is a subset of the standard library (`std`) that is designed for use in
/// environments that do not have access to the full standard library. It provides essential
/// functionalities such as basic data types, traits, and macros. By declaring `extern crate core`,
/// we are explicitly stating that we want to use the `core` crate in our project.
extern crate core;

/// The `gdelt_fetcher` module is responsible for fetching data from the GDELT (Global Database of
/// Events, Language, and Tone) project. This module likely contains functions and structures to
/// handle the retrieval of data files, parsing of data, and possibly caching mechanisms to store
/// fetched data for later use.
pub mod gdelt_fetcher;

/// The `csv_to_json` module is designed to handle the conversion of CSV (Comma-Separated Values)
/// data into JSON (JavaScript Object Notation) format. This module likely includes functions and
/// structures to read CSV files, parse the data, and then serialize it into JSON format, which is
/// widely used for data interchange.
pub mod csv_to_json;

/// The `utils` module contains utility functions, types, and other helper code that is used
/// throughout the project. This module is a common place to put code that is shared across
/// multiple modules, such as custom data types, error handling utilities, and other general-purpose
/// functions that do not belong to a specific module.
pub mod utils;

/// The `csv_to_structs` module is responsible for converting CSV data into Rust structs. This
/// module likely includes functions and structures to read CSV files, parse the data, and then
/// populate Rust structs with the parsed data. This is useful for working with structured data
/// in a type-safe manner within the Rust programming language.

//TODO: Parallelisation

pub mod csv_to_structs;
pub mod data_reader;
pub mod gdelt_api;
