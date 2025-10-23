//! HTTP fetcher for in-memory data retrieval
//!
//! This module provides WASM-compatible HTTP fetching that works entirely in memory,
//! without requiring file system access.

use anyhow::{Context, Result};

/// Fetch content from a URL into memory
#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_to_memory(url: &str) -> Result<Vec<u8>> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to fetch URL: {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP error {}: {}", response.status(), url);
    }

    let bytes = response
        .bytes()
        .await
        .with_context(|| format!("Failed to read response body from: {}", url))?;

    Ok(bytes.to_vec())
}

/// Fetch content from a URL into memory (WASM version)
#[cfg(target_arch = "wasm32")]
pub async fn fetch_to_memory(url: &str) -> Result<Vec<u8>> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| anyhow::anyhow!("Failed to create request: {:?}", e))?;

    let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("No window object"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Fetch failed: {:?}", e))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|e| anyhow::anyhow!("Failed to cast to Response: {:?}", e))?;

    if !resp.ok() {
        anyhow::bail!("HTTP error {}: {}", resp.status(), url);
    }

    let array_buffer = JsFuture::from(
        resp.array_buffer()
            .map_err(|e| anyhow::anyhow!("Failed to get array buffer: {:?}", e))?,
    )
    .await
    .map_err(|e| anyhow::anyhow!("Failed to read array buffer: {:?}", e))?;

    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    let mut bytes = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut bytes);

    Ok(bytes)
}

/// Fetch text content from a URL
pub async fn fetch_text(url: &str) -> Result<String> {
    let bytes = fetch_to_memory(url).await?;
    String::from_utf8(bytes).with_context(|| format!("Invalid UTF-8 in response from: {}", url))
}

/// Decompress gzip data in memory
pub fn decompress_gzip(compressed: &[u8]) -> Result<Vec<u8>> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .with_context(|| "Failed to decompress gzip data")?;

    Ok(decompressed)
}

/// Extract a single file from a ZIP archive in memory
pub fn extract_from_zip(zip_data: &[u8], expected_extension: &str) -> Result<Vec<u8>> {
    use std::io::Read;
    use zip::ZipArchive;

    let cursor = std::io::Cursor::new(zip_data);
    let mut archive = ZipArchive::new(cursor)
        .with_context(|| "Failed to read ZIP archive")?;

    // Find the first file with the expected extension
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .with_context(|| format!("Failed to access file at index {}", i))?;

        if file.name().ends_with(expected_extension) {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .with_context(|| format!("Failed to read file: {}", file.name()))?;
            return Ok(contents);
        }
    }

    anyhow::bail!("No file with extension '{}' found in ZIP archive", expected_extension)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_gzip() {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let original = b"Hello, World! This is a test string.";
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(original).unwrap();
        let compressed = encoder.finish().unwrap();

        let decompressed = decompress_gzip(&compressed).unwrap();
        assert_eq!(&decompressed, original);
    }

    #[test]
    fn test_extract_from_zip() {
        use std::io::Write;
        use zip::write::{FileOptions, ZipWriter};

        // Create a test ZIP archive in memory
        let mut zip_buffer = std::io::Cursor::new(Vec::new());
        {
            let mut zip = ZipWriter::new(&mut zip_buffer);
            zip.start_file::<_, ()>("test.csv", FileOptions::default()).unwrap();
            zip.write_all(b"col1,col2\nval1,val2").unwrap();
            zip.finish().unwrap();
        }

        let zip_data = zip_buffer.into_inner();
        let extracted = extract_from_zip(&zip_data, "csv").unwrap();
        assert_eq!(&extracted, b"col1,col2\nval1,val2");
    }
}
