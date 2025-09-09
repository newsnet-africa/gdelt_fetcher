# GDELT Fetcher Implementation Guide

This document provides technical approaches and implementation strategies for completing the TODO items in the GDELT Fetcher crate.

## Critical Implementation Challenges

### 1. Fix GCAM Codebook Indexing (Highest Priority)

**Current Problem**: Using a "super weird hack" for GCAM code lookup that needs to be replaced with a proper implementation.

**Current State Analysis**:
- GCAM codes are loaded from CSV but using inefficient lookup mechanism
- Need to replace with proper key-value store or in-memory structure
- Must handle missing codes gracefully

#### Implementation Strategy

**Option A: Build-Time Generation (Recommended)**
```rust
// In build.rs
use std::collections::HashMap;
use csv::Reader;

fn generate_gcam_lookup() -> Result<()> {
    let mut file = File::create("src/gcam/generated_lookup.rs")?;
    let mut reader = csv::Reader::from_path("GCAM-MASTER-CODEBOOK-fixed.csv")?;
    
    writeln!(file, "use std::collections::HashMap;")?;
    writeln!(file, "use once_cell::sync::Lazy;")?;
    writeln!(file, "")?;
    writeln!(file, "pub static GCAM_LOOKUP: Lazy<HashMap<&'static str, GcamCode>> = Lazy::new(|| {")?;
    writeln!(file, "    let mut map = HashMap::new();")?;
    
    for result in reader.records() {
        let record = result?;
        let code = &record[0];
        let description = &record[1];
        let category = &record[2];
        
        writeln!(file, "    map.insert(\"{}\", GcamCode {{", code)?;
        writeln!(file, "        code: \"{}\",", code)?;
        writeln!(file, "        description: \"{}\",", description.replace("\"", "\\\""))?;
        writeln!(file, "        category: \"{}\",", category)?;
        writeln!(file, "    }});")?;
    }
    
    writeln!(file, "    map")?;
    writeln!(file, "}});")?;
    
    Ok(())
}
```

**Option B: Runtime Lazy Loading with Once Cell**
```rust
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct GcamCode {
    pub code: String,
    pub description: String,
    pub category: String,
    pub weight: Option<f32>,
}

static GCAM_LOOKUP: Lazy<HashMap<String, GcamCode>> = Lazy::new(|| {
    load_gcam_codes().unwrap_or_else(|e| {
        log::error!("Failed to load GCAM codes: {}", e);
        HashMap::new()
    })
});

fn load_gcam_codes() -> anyhow::Result<HashMap<String, GcamCode>> {
    let csv_content = include_str!("../GCAM-MASTER-CODEBOOK-fixed.csv");
    let mut reader = csv::Reader::from_reader(csv_content.as_bytes());
    let mut codes = HashMap::new();
    
    for result in reader.records() {
        let record = result?;
        let code = record.get(0).ok_or(anyhow!("Missing code column"))?;
        let description = record.get(1).unwrap_or("");
        let category = record.get(2).unwrap_or("");
        let weight = record.get(3)
            .and_then(|s| s.parse::<f32>().ok());
        
        codes.insert(code.to_string(), GcamCode {
            code: code.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            weight,
        });
    }
    
    Ok(codes)
}

pub fn lookup_gcam_code(code: &str) -> Option<&GcamCode> {
    GCAM_LOOKUP.get(code)
}

pub fn validate_gcam_enrichment(gkg_record: &GKGTable) -> ValidationResult {
    let mut issues = Vec::new();
    
    // Check if GCAM codes exist and are valid
    if let Some(gcam_data) = &gkg_record.gcam {
        for gcam_entry in &gcam_data.entries {
            if lookup_gcam_code(&gcam_entry.code).is_none() {
                issues.push(ValidationIssue::InvalidGcamCode {
                    code: gcam_entry.code.clone(),
                    context: "GKG GCAM data".to_string(),
                });
            }
        }
    }
    
    ValidationResult { issues }
}
```

**Technical Considerations**:
- Use `phf` crate for perfect hashing if performance is critical
- Consider memory usage vs lookup speed tradeoffs
- Add metrics for cache hit/miss rates
- Handle encoding issues in CSV data

### 2. Complete HTTP Client Implementation

**Current Problem**: No actual HTTP implementation, just placeholder functions.

**Implementation Strategy**:

```rust
use reqwest::{Client, Response};
use tokio::time::{sleep, Duration, Instant};
use std::sync::Arc;

#[derive(Clone)]
pub struct GdeltHttpClient {
    client: Client,
    rate_limiter: Arc<RateLimiter>,
    retry_config: RetryConfig,
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

impl GdeltHttpClient {
    pub fn new() -> anyhow::Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent("NewsNet-GDELT-Fetcher/1.0")
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build()?;
        
        Ok(Self {
            client,
            rate_limiter: Arc::new(RateLimiter::new(10.0, 50)), // 10 req/sec, burst 50
            retry_config: RetryConfig::default(),
            base_url: "http://data.gdeltproject.org".to_string(),
        })
    }
    
    pub async fn download_with_retry(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        let mut attempt = 0;
        let mut delay = self.retry_config.initial_delay;
        
        loop {
            attempt += 1;
            
            // Rate limiting
            self.rate_limiter.wait().await;
            
            match self.download_once(url).await {
                Ok(data) => return Ok(data),
                Err(e) if attempt >= self.retry_config.max_attempts => {
                    return Err(anyhow!("Failed after {} attempts: {}", attempt, e));
                }
                Err(e) => {
                    log::warn!("Attempt {} failed for {}: {}", attempt, url, e);
                    
                    // Exponential backoff with jitter
                    let jitter = rand::random::<f32>() * 0.1; // 10% jitter
                    let sleep_duration = delay + Duration::from_secs_f32(delay.as_secs_f32() * jitter);
                    
                    sleep(sleep_duration).await;
                    
                    delay = Duration::min(
                        Duration::from_secs_f32(delay.as_secs_f32() * self.retry_config.backoff_multiplier),
                        self.retry_config.max_delay
                    );
                }
            }
        }
    }
    
    async fn download_once(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        let response = self.client
            .get(url)
            .header("Accept-Encoding", "gzip, deflate")
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }
        
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
    
    pub async fn download_with_progress<F>(&self, url: &str, mut progress_callback: F) -> anyhow::Result<Vec<u8>>
    where
        F: FnMut(u64, Option<u64>) + Send + Sync,
    {
        self.rate_limiter.wait().await;
        
        let response = self.client.get(url).send().await?;
        let total_size = response.content_length();
        
        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();
        let mut data = Vec::new();
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            data.extend_from_slice(&chunk);
            downloaded += chunk.len() as u64;
            progress_callback(downloaded, total_size);
        }
        
        Ok(data)
    }
    
    pub async fn get_last_update_info(&self) -> anyhow::Result<LastUpdateInfo> {
        let url = format!("{}/api/v2/lastupdate", self.base_url);
        let response = self.download_with_retry(&url).await?;
        let text = String::from_utf8(response)?;
        
        // Parse GDELT lastupdate format
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() < 3 {
            return Err(anyhow!("Invalid lastupdate format"));
        }
        
        Ok(LastUpdateInfo {
            events_url: lines[0].to_string(),
            mentions_url: lines[1].to_string(),
            gkg_url: lines[2].to_string(),
            timestamp: parse_gdelt_timestamp(lines[0])?,
        })
    }
}

// Rate limiter implementation
pub struct RateLimiter {
    rate: f64,
    capacity: u32,
    tokens: Arc<Mutex<f64>>,
    last_update: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    pub fn new(rate: f64, capacity: u32) -> Self {
        Self {
            rate,
            capacity,
            tokens: Arc::new(Mutex::new(capacity as f64)),
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }
    
    pub async fn wait(&self) {
        loop {
            let now = Instant::now();
            let mut tokens = self.tokens.lock().await;
            let mut last_update = self.last_update.lock().await;
            
            // Add tokens based on elapsed time
            let elapsed = now.duration_since(*last_update).as_secs_f64();
            *tokens = (*tokens + elapsed * self.rate).min(self.capacity as f64);
            *last_update = now;
            
            if *tokens >= 1.0 {
                *tokens -= 1.0;
                break;
            }
            
            // Wait before trying again
            let wait_time = Duration::from_secs_f64((1.0 - *tokens) / self.rate);
            drop(tokens);
            drop(last_update);
            sleep(wait_time).await;
        }
    }
}
```

### 3. File Processing and ZIP Extraction

**Implementation Strategy**:

```rust
use async_compression::tokio::bufread::GzipDecoder;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use zip::ZipArchive;

pub struct FileProcessor {
    temp_dir: PathBuf,
    output_dir: PathBuf,
}

impl FileProcessor {
    pub fn new<P: AsRef<Path>>(temp_dir: P, output_dir: P) -> Self {
        Self {
            temp_dir: temp_dir.as_ref().to_path_buf(),
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }
    
    pub async fn process_downloaded_file(&self, data: Vec<u8>, filename: &str) -> anyhow::Result<ProcessedFile> {
        // Verify file integrity
        let expected_hash = self.get_expected_hash(filename).await?;
        let actual_hash = calculate_sha256(&data);
        
        if expected_hash != actual_hash {
            return Err(anyhow!("File integrity check failed for {}", filename));
        }
        
        // Save to temp directory
        let temp_path = self.temp_dir.join(filename);
        tokio::fs::write(&temp_path, &data).await?;
        
        // Process based on file type
        let processed = if filename.ends_with(".zip") {
            self.extract_zip(temp_path).await?
        } else if filename.ends_with(".gz") {
            self.decompress_gzip(temp_path).await?
        } else {
            // Plain CSV file
            ProcessedFile {
                path: temp_path,
                format: FileFormat::Csv,
                records_count: None,
            }
        };
        
        Ok(processed)
    }
    
    async fn extract_zip(&self, zip_path: PathBuf) -> anyhow::Result<ProcessedFile> {
        let file = std::fs::File::open(&zip_path)?;
        let mut archive = ZipArchive::new(file)?;
        
        // GDELT ZIP files typically contain a single CSV file
        if archive.len() != 1 {
            return Err(anyhow!("Expected single file in ZIP, found {}", archive.len()));
        }
        
        let mut zip_file = archive.by_index(0)?;
        let output_path = self.temp_dir.join(zip_file.name());
        
        let mut output_file = std::fs::File::create(&output_path)?;
        std::io::copy(&mut zip_file, &mut output_file)?;
        
        Ok(ProcessedFile {
            path: output_path,
            format: FileFormat::Csv,
            records_count: None,
        })
    }
    
    async fn decompress_gzip(&self, gzip_path: PathBuf) -> anyhow::Result<ProcessedFile> {
        let file = File::open(&gzip_path).await?;
        let buf_reader = BufReader::new(file);
        let mut decoder = GzipDecoder::new(buf_reader);
        
        let output_path = self.temp_dir.join(
            gzip_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("decompressed.csv")
        );
        
        let mut output_file = File::create(&output_path).await?;
        tokio::io::copy(&mut decoder, &mut output_file).await?;
        
        Ok(ProcessedFile {
            path: output_path,
            format: FileFormat::Csv,
            records_count: None,
        })
    }
    
    pub async fn stream_csv_records<T>(&self, file_path: &Path) -> anyhow::Result<impl Stream<Item = anyhow::Result<T>>>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        let file = File::open(file_path).await?;
        let buf_reader = BufReader::new(file);
        let mut csv_reader = csv_async::AsyncReader::from_reader(buf_reader);
        
        Ok(csv_reader.into_deserialize::<T>().map(|result| {
            result.map_err(|e| anyhow!("CSV parsing error: {}", e))
        }))
    }
    
    async fn get_expected_hash(&self, filename: &str) -> anyhow::Result<String> {
        // GDELT provides MD5 hashes for verification
        // This would need to be implemented based on GDELT's hash distribution method
        // For now, return a placeholder
        Ok("placeholder_hash".to_string())
    }
    
    pub async fn cleanup_temp_files(&self) -> anyhow::Result<()> {
        let mut dir = tokio::fs::read_dir(&self.temp_dir).await?;
        
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                tokio::fs::remove_file(&path).await?;
            }
        }
        
        Ok(())
    }
}

fn calculate_sha256(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[derive(Debug)]
pub struct ProcessedFile {
    pub path: PathBuf,
    pub format: FileFormat,
    pub records_count: Option<usize>,
}

#[derive(Debug)]
pub enum FileFormat {
    Csv,
    Json,
    Binary,
}
```

### 4. Complete DataFetcher Implementation

**Current Problem**: Trait is defined but implementations are empty placeholders.

```rust
use async_trait::async_trait;
use futures::Stream;
use chrono::NaiveDate;

#[async_trait]
impl<T> DataFetcher<T> for GdeltFetcher
where
    T: for<'de> Deserialize<'de> + Send + 'static,
{
    async fn fetch_latest_async(&mut self) -> anyhow::Result<Box<dyn Iterator<Item = T>>> {
        let http_client = GdeltHttpClient::new()?;
        let file_processor = FileProcessor::new(&self.temp_dir, &self.output_dir);
        
        // Get latest update info
        let update_info = http_client.get_last_update_info().await?;
        
        // Determine which URL to use based on table type
        let url = match self.table_type {
            TableType::Events => update_info.events_url,
            TableType::Mentions => update_info.mentions_url,
            TableType::GKG => update_info.gkg_url,
        };
        
        // Download file
        let filename = url.split('/').last().unwrap_or("download.zip");
        log::info!("Downloading {} from {}", filename, url);
        
        let data = http_client.download_with_retry(&url).await?;
        log::info!("Downloaded {} bytes", data.len());
        
        // Process file (extract, decompress)
        let processed_file = file_processor.process_downloaded_file(data, filename).await?;
        log::info!("Processed file: {:?}", processed_file.path);
        
        // Parse CSV and collect into iterator
        let mut records = Vec::new();
        let mut csv_stream = file_processor.stream_csv_records::<T>(&processed_file.path).await?;
        
        while let Some(record) = csv_stream.next().await {
            match record {
                Ok(parsed) => records.push(parsed),
                Err(e) => {
                    log::warn!("Failed to parse record: {}", e);
                    // Continue processing other records
                }
            }
        }
        
        log::info!("Parsed {} records", records.len());
        
        // Cleanup temp files
        file_processor.cleanup_temp_files().await?;
        
        Ok(Box::new(records.into_iter()))
    }
    
    async fn fetch_date_async(&mut self, date: NaiveDate) -> anyhow::Result<Box<dyn Iterator<Item = T>>> {
        let http_client = GdeltHttpClient::new()?;
        let file_processor = FileProcessor::new(&self.temp_dir, &self.output_dir);
        
        // Construct URL for specific date
        let date_str = date.format("%Y%m%d").to_string();
        let hour_str = "000000"; // Start with midnight, could be parameterized
        
        let filename = match self.table_type {
            TableType::Events => format!("{}{}.export.CSV.zip", date_str, hour_str),
            TableType::Mentions => format!("{}{}.mentions.CSV.zip", date_str, hour_str),
            TableType::GKG => format!("{}{}.gkg.csv.zip", date_str, hour_str),
        };
        
        let url = format!("http://data.gdeltproject.org/events/{}", filename);
        
        log::info!("Fetching historical data from {}", url);
        
        // Similar processing as fetch_latest_async
        let data = http_client.download_with_retry(&url).await?;
        let processed_file = file_processor.process_downloaded_file(data, &filename).await?;
        
        let mut records = Vec::new();
        let mut csv_stream = file_processor.stream_csv_records::<T>(&processed_file.path).await?;
        
        while let Some(record) = csv_stream.next().await {
            match record {
                Ok(parsed) => records.push(parsed),
                Err(e) => log::warn!("Failed to parse record: {}", e),
            }
        }
        
        file_processor.cleanup_temp_files().await?;
        
        Ok(Box::new(records.into_iter()))
    }
    
    fn output_dir(&self) -> &str {
        &self.output_dir
    }
    
    fn temp_dir(&self) -> &str {
        &self.temp_dir
    }
}
```

### 5. BigQuery Integration Implementation

**Strategy**: Start with REST API, add streaming later

```rust
use google_cloud_auth::{Credentials, TokenSource};
use serde_json::Value;

pub struct BigQueryClient {
    client: reqwest::Client,
    project_id: String,
    dataset_id: String,
    credentials: Arc<dyn TokenSource>,
}

impl BigQueryClient {
    pub async fn new(project_id: String, service_account_path: &Path) -> anyhow::Result<Self> {
        let credentials = Credentials::from_file(service_account_path).await?;
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minutes for large queries
            .build()?;
        
        Ok(Self {
            client,
            project_id,
            dataset_id: "gdelt-bq".to_string(),
            credentials: Arc::new(credentials),
        })
    }
    
    pub async fn query_events(&self, start_date: NaiveDate, end_date: NaiveDate) -> anyhow::Result<impl Stream<Item = EventTable>> {
        let sql = format!(
            "SELECT * FROM `{}.events` WHERE DATE(_PARTITIONTIME) BETWEEN '{}' AND '{}' ORDER BY dateadded DESC",
            self.dataset_id,
            start_date.format("%Y-%m-%d"),
            end_date.format("%Y-%m-%d")
        );
        
        self.execute_streaming_query(sql).await
    }
    
    async fn execute_streaming_query<T>(&self, sql: String) -> anyhow::Result<impl Stream<Item = T>>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        // Start query job
        let job_id = self.start_query_job(sql).await?;
        
        // Wait for completion
        self.wait_for_job_completion(&job_id).await?;
        
        // Stream results
        self.stream_query_results(&job_id).await
    }
    
    async fn start_query_job(&self, sql: String) -> anyhow::Result<String> {
        let token = self.credentials.access_token().await?;
        
        let job_config = serde_json::json!({
            "configuration": {
                "query": {
                    "query": sql,
                    "useLegacySql": false,
                    "maximumBytesBilled": "1000000000" // 1GB limit
                }
            }
        });
        
        let url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/jobs",
            self.project_id
        );
        
        let response = self.client
            .post(&url)
            .bearer_auth(token.access_token())
            .json(&job_config)
            .send()
            .await?;
        
        let job_response: Value = response.json().await?;
        let job_id = job_response["jobReference"]["jobId"]
            .as_str()
            .ok_or_else(|| anyhow!("No job ID in response"))?;
        
        Ok(job_id.to_string())
    }
    
    async fn wait_for_job_completion(&self, job_id: &str) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(Duration::from_secs(2));
        let timeout = tokio::time::sleep(Duration::from_secs(300)); // 5 minute timeout
        
        tokio::pin!(timeout);
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if self.is_job_complete(job_id).await? {
                        return Ok(());
                    }
                }
                _ = &mut timeout => {
                    return Err(anyhow!("Query timeout after 5 minutes"));
                }
            }
        }
    }
    
    async fn is_job_complete(&self, job_id: &str) -> anyhow::Result<bool> {
        let token = self.credentials.access_token().await?;
        let url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/jobs/{}",
            self.project_id, job_id
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(token.access_token())
            .send()
            .await?;
        
        let job_status: Value = response.json().await?;
        let state = job_status["status"]["state"]
            .as_str()
            .unwrap_or("UNKNOWN");
        
        match state {
            "DONE" => Ok(true),
            "RUNNING" | "PENDING" => Ok(false),
            _ => Err(anyhow!("Job failed with state: {}", state)),
        }
    }
    
    async fn stream_query_results<T>(&self, job_id: &str) -> anyhow::Result<impl Stream<Item = T>>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        // Implementation would stream paginated results from BigQuery
        // This is a simplified version
        todo!("Implement result streaming")
    }
}
```

## Testing Strategy

### Integration Testing with Real GDELT Data

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_full_fetch_workflow() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_dir = tempfile::tempdir().unwrap();
        
        let mut fetcher = EventTableFetcher::new_v2(
            output_dir.path().to_str().unwrap(),
            temp_dir.path().to_str().unwrap(),
        ).unwrap();
        
        // Test with a small historical dataset
        let test_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let events = fetcher.fetch_date_async(test_date).await.unwrap();
        
        let event_count = events.count();
        assert!(event_count > 0, "Should fetch some events");
    }
    
    #[tokio::test]
    async fn test_http_client_retry() {
        let client = GdeltHttpClient::new().unwrap();
        
        // Test with a known bad URL to trigger retry
        let result = client.download_with_retry("http://httpstat.us/500").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_gcam_lookup_performance() {
        let start = std::time::Instant::now();
        
        // Test lookup performance
        for _ in 0..10000 {
            let _ = lookup_gcam_code("c01.01");
        }
        
        let elapsed = start.elapsed();
        assert!(elapsed < Duration::from_millis(10), "GCAM lookup should be fast");
    }
}
```

### Mock Testing for Development

```rust
pub struct MockHttpClient {
    responses: HashMap<String, Vec<u8>>,
}

impl MockHttpClient {
    pub fn new() -> Self {
        let mut responses = HashMap::new();
        responses.insert(
            "http://data.gdeltproject.org/api/v2/lastupdate".to_string(),
            include_bytes!("../test_data/mock_lastupdate.txt").to_vec(),
        );
        responses.insert(
            "test_events_url".to_string(),
            include_bytes!("../test_data/mock_events.csv.zip").to_vec(),
        );
        
        Self { responses }
    }
}

#[async_trait]
impl HttpClient for MockHttpClient {
    async fn download_with_retry(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        self.responses.get(url)
            .cloned()
            .ok_or_else(|| anyhow!("Mock data not found for URL: {}", url))
    }
}
```

## Performance Optimization Strategies

### Memory Management

```rust
// Use streaming instead of loading entire datasets
pub async fn process_large_file_streaming<T, F>(
    file_path: &Path,
    mut processor: F,
) -> anyhow::Result<()>
where
    T: for<'de> Deserialize<'de>,
    F: FnMut(T) -> anyhow::Result<()>,
{
    let file = File::open(file_path).await?;
    let buf_reader = BufReader::with_capacity(64 * 1024, file); // 64KB buffer
    let mut csv_reader = csv_async::AsyncReader::from_reader(buf_reader);
    
    let mut stream = csv_reader.into_deserialize::<T>();
    while let Some(record) = stream.next().await {
        let parsed = record?;
        processor(parsed)?;
    }
    
    Ok(())
}
```

### Parallel Processing

```rust
use rayon::prelude::*;

pub fn process_records_parallel<T, F, R>(
    records: Vec<T>,
    processor: F,
) -> Vec<anyhow::Result<R>>
where
    T: Send + Sync,
    F: Fn(&T) -> anyhow::Result<R> + Send + Sync,
    R: Send,
{
    records
        .par_iter()
        .map(|record| processor(record))
        .collect()
}
```

## Monitoring and Observability

```rust
use metrics::{counter, histogram, gauge};

pub struct FetcherMetrics {
    downloads_total: counter::Counter,
    download_duration: histogram::Histogram,
    records_processed: gauge::Gauge,
}

impl FetcherMetrics {
    pub fn new() -> Self {
        Self {
            downloads_total: metrics::counter!("gdelt_fetcher_downloads_total"),
            download_duration: metrics::histogram!("gdelt_fetcher_download_duration_seconds"),
            records_processed: metrics::gauge!("gdelt_fetcher_records_processed_total"),
        }
    }
    
    pub fn record_download(&self, duration: Duration) {
        self.downloads_total.increment(1);
        self.download_duration.record(duration.as_secs_f64());
    }
}
```

This implementation guide provides concrete technical approaches for the major TODO items in the gdelt_fetcher crate. Focus on the GCAM fix first as it's blocking other functionality, then implement the HTTP client as it's needed for all data fetching operations.