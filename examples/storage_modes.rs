//! Example demonstrating different storage modes for CSV data
//!
//! This example shows how to configure the fetcher to:
//! 1. Process data entirely in memory (default)
//! 2. Save CSV files to system tmp directory
//! 3. Save CSV files to a custom path
//!
//! Note: Storage modes are only available on native targets (not WASM)

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("=== Storage Mode Examples ===\n");

    // Example 1: In-Memory Processing (default, works on all platforms including WASM)
    println!("--- Example 1: In-Memory Processing ---");
    println!("Processing data entirely in memory without saving to disk");
    println!("This mode is the default and works on all platforms including WASM\n");

    let mut fetcher_memory = data::GdeltFetcher::new();
    fetcher_memory.initialize().await?;

    let events = fetcher_memory.fetch_latest_events().await?;
    println!("✓ Fetched {} events (in-memory only, no files saved)", events.len());
    println!("  Storage mode: {:?}\n", fetcher_memory.storage_mode());

    // Example 2: Temporary Directory Storage (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("--- Example 2: Temporary Directory Storage ---");
        println!("Saving CSV files to system tmp directory");

        let mut fetcher_tmp = data::GdeltFetcher::with_tmp_storage();
        fetcher_tmp.initialize().await?;

        let events = fetcher_tmp.fetch_latest_events().await?;
        println!("✓ Fetched {} events and saved CSV to tmp directory", events.len());
        println!("  Storage mode: {:?}", fetcher_tmp.storage_mode());
        println!("  Files saved to: {}/gdelt/", std::env::temp_dir().display());

        let mentions = fetcher_tmp.fetch_latest_mentions().await?;
        println!("✓ Fetched {} mentions and saved CSV to tmp directory", mentions.len());

        let gkg = fetcher_tmp.fetch_latest_gkg().await?;
        println!("✓ Fetched {} GKG records and saved CSV to tmp directory\n", gkg.len());
    }

    // Example 3: Custom Path Storage (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("--- Example 3: Custom Path Storage ---");
        println!("Saving CSV files to a custom directory");

        let custom_path = "./data/gdelt_custom";
        let mut fetcher_custom = data::GdeltFetcher::with_custom_path(custom_path);
        fetcher_custom.initialize().await?;

        let events = fetcher_custom.fetch_latest_events().await?;
        println!("✓ Fetched {} events and saved CSV to custom path", events.len());
        println!("  Storage mode: {:?}", fetcher_custom.storage_mode());
        println!("  Files saved to: {}/", custom_path);

        let mentions = fetcher_custom.fetch_latest_mentions().await?;
        println!("✓ Fetched {} mentions and saved CSV to custom path", mentions.len());

        let gkg = fetcher_custom.fetch_latest_gkg().await?;
        println!("✓ Fetched {} GKG records and saved CSV to custom path\n", gkg.len());
    }

    // Example 4: Changing Storage Mode at Runtime (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("--- Example 4: Changing Storage Mode at Runtime ---");
        println!("You can change the storage mode after creating the fetcher");

        let mut fetcher = data::GdeltFetcher::new();
        println!("Initial storage mode: {:?}", fetcher.storage_mode());

        // Change to tmp directory mode
        fetcher.set_storage_mode(data::StorageMode::TmpDirectory);
        println!("Changed to: {:?}", fetcher.storage_mode());

        // Change to custom path mode
        fetcher.set_storage_mode(data::StorageMode::CustomPath("./data/runtime_change".into()));
        println!("Changed to: {:?}", fetcher.storage_mode());

        // Change back to in-memory mode
        fetcher.set_storage_mode(data::StorageMode::InMemory);
        println!("Changed to: {:?}\n", fetcher.storage_mode());
    }

    // Example 5: Using StorageMode enum directly (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("--- Example 5: Using StorageMode Enum Directly ---");

        // You can create storage modes explicitly
        let modes = vec![
            data::StorageMode::InMemory,
            data::StorageMode::TmpDirectory,
            data::StorageMode::CustomPath("./data/explicit_path".into()),
        ];

        for (i, mode) in modes.iter().enumerate() {
            println!("Storage mode {}: {:?}", i + 1, mode);
            let fetcher = data::GdeltFetcher::with_storage_mode(mode.clone());
            println!("  Created fetcher with storage mode: {:?}", fetcher.storage_mode());
        }
    }

    println!("\n=== Storage Mode Examples Completed ===");
    println!("\nKey Takeaways:");
    println!("• InMemory (default): Works on all platforms, no disk usage");
    println!("• TmpDirectory: Saves to system tmp dir, good for testing");
    println!("• CustomPath: Full control over where CSV files are saved");
    println!("• Storage modes can be changed at runtime");
    println!("• Saved CSV files can be reused or analyzed separately");

    Ok(())
}
