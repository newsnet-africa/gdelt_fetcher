use csv::{ReaderBuilder, StringRecord};
use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

pub fn data_reader(download_path: PathBuf, delimited: bool) -> Result<Vec<StringRecord>, Error> {
    let file = File::open(download_path)?;

    let mut reader = if delimited {
        ReaderBuilder::new()
            .flexible(true)
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(file)
    } else {
        ReaderBuilder::new()
            .flexible(true)
            .has_headers(false)
            .from_reader(file)
    };

    let mut records: Vec<StringRecord> = reader.records().collect::<Result<_, _>>()?;

    // Remove the first item from the vector
    if !records.is_empty() {
        records.remove(0);
    }

    for record in &records {
        println!("{:?}", record);
    }

    Ok(records)
}