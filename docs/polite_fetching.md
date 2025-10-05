# Polite Fetching: Local File Checking

The GDELT Fetcher now includes intelligent local file checking to avoid unnecessary API calls when data already exists locally. This feature makes development faster, reduces server load, and enables offline work after initial data downloads.

## How It Works

Before making any API call, the fetcher now:

1. **Checks the output directory** (`./data/{type}/`) for existing files
2. **Falls back to the tmp directory** (`./tmp/{type}/`) if not found in output
3. **Only makes an API call** if the file doesn't exist locally
4. **Returns cached data** if a matching file is found

## Benefits

- ✅ **Faster development iteration** - Subsequent calls use cached data
- ✅ **Reduced server load** - Being polite to GDELT servers  
- ✅ **Offline capability** - Work with previously downloaded data
- ✅ **Automatic cache management** - No configuration required

## File Matching Logic

### Latest Data Files
For "latest" requests, the fetcher looks for any file containing the table identifier:
- Events: Files containing `export` and ending with `.csv`
- Mentions: Files containing `mentions` and ending with `.csv`  
- GKG: Files containing `gkg` and ending with `.csv`

### Date-Specific Files
For date-specific requests, the fetcher matches exact timestamps:
- Pattern: `YYYYMMDDHHMMSS.{table_type}.csv`
- Example: `20240115000000.export.csv`

## Example Usage

```rust
use gdelt_fetcher::api::fetch_latest_events;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // First call - downloads from API
    let events1 = fetch_latest_events().await?;
    println!("Downloaded {} events", events1.len());
    
    // Second call - uses cached file (much faster!)
    let events2 = fetch_latest_events().await?;
    println!("Loaded {} events from cache", events2.len());
    
    Ok(())
}
```

## Observing the Behavior

Enable logging to see the polite fetching in action:

```rust
env_logger::Builder::from_default_env()
    .filter_level(log::LevelFilter::Info)
    .init();
```

You'll see messages like:
- `📁 Found existing file in output directory: ./data/events/file.csv`
- `📋 Using existing events file, skipping download for politeness`
- `✅ Loaded 1000 events from existing file in 50ms`

## Directory Structure

The fetcher manages two directories:

```
./data/           # Final processed files
├── events/       # Event table CSV files
├── mentions/     # Mention table CSV files  
└── gkg/          # GKG table CSV files

./tmp/            # Temporary download staging
├── events/       # Temporary event files
├── mentions/     # Temporary mention files
└── gkg/          # Temporary GKG files
```

## Cache Invalidation

Currently, the cache is persistent - files remain until manually deleted. For fresh data:

1. **Delete specific files** from `./data/{type}/` to re-download
2. **Clear entire cache** by removing `./data/` and `./tmp/` directories
3. **Use different dates** for date-specific requests

## API Functions Affected

All API functions now include local file checking:

- `fetch_latest_events()`
- `fetch_latest_mentions()`  
- `fetch_latest_gkg()`
- `fetch_events_by_date()`
- `fetch_mentions_by_date()`
- `fetch_gkg_by_date()`
- `fetch_all_latest()` (via individual functions)
- `fetch_all_by_date()` (via individual functions)

## Performance Impact

- **Cache hit**: ~10-50ms (file I/O only)
- **Cache miss**: Variable (depends on download size and network)
- **Memory**: No additional memory overhead
- **Storage**: Files persist in local directories

## Implementation Details

The feature adds these helper functions to the API:

- `check_existing_file()` - Main file existence check
- `search_directory_for_file()` - Directory scanning logic  
- `load_*_from_file()` - File loading functions

These functions use the existing `TableType` and file extension logic from the underlying GDELT fetcher to ensure compatibility.

## Troubleshooting

### File Not Found Despite Existing
- Ensure filenames match expected patterns
- Check file extensions (must be `.csv`)
- Verify directory permissions

### Stale Cache Data
- Delete specific files to force refresh
- Check file timestamps if needed
- Clear directories for complete refresh

### Logging Issues
- Enable `log::LevelFilter::Info` or `Debug` to see detailed behavior
- Use `RUST_LOG=info` environment variable for quick debugging

## Future Enhancements

Potential improvements being considered:

- TTL-based cache expiration
- Configurable cache behavior
- Cache statistics and management
- Checksum-based cache validation