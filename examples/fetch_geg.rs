use anyhow::Result;
use data::fetchers::gdelt::GEGFetcher;
use models::types::geg_table::GEGTable;
use std::env;

/// Fetch and display GDELT 3.0 GEG (GDELT Event Graph) entries
///
/// This example demonstrates fetching entity-extracted news articles from GDELT 3.0
/// processed through Google's Cloud Natural Language API.
///
/// Usage: cargo run --example fetch_geg [count]
///
/// Arguments:
///   count - Number of items to display (default: 5)
///
/// Example:
///   cargo run --example fetch_geg 10
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Get count from command line args, default to 5
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(5)
    } else {
        5
    };

    println!("{}", "=".repeat(80));
    println!("GDELT 3.0 Event Graph (GEG) Fetcher");
    println!("{}", "=".repeat(80));
    println!("Setting up GEG fetcher...\n");

    // Create fetcher with a temporary store directory
    let store_dir = "./tmp/geg_store";
    let fetcher = GEGFetcher::new(store_dir)?;

    println!("Initializing masterfilelist (this may take a moment on first run)...");
    let init_count = fetcher.initialize().await?;
    println!("✅ Initialized with {} masterlist entries\n", init_count);

    println!("Fetching latest GEG data...");
    let geg_entries = fetcher.fetch_latest().await?;

    println!("✅ Successfully fetched {} GEG entries", geg_entries.len());
    println!("📊 Displaying first {} entries:\n", count);

    for (i, geg) in geg_entries.iter().take(count).enumerate() {
        print_geg(i + 1, geg);
    }

    println!("\n{}", "=".repeat(80));
    println!("Summary:");
    println!("  Total GEG entries fetched: {}", geg_entries.len());
    println!("  Entries displayed: {}", count.min(geg_entries.len()));

    // Calculate some statistics
    let total_entities: usize = geg_entries.iter().map(|e| e.entities.len()).sum();
    let avg_entities = if !geg_entries.is_empty() {
        total_entities as f64 / geg_entries.len() as f64
    } else {
        0.0
    };

    println!("  Total entities extracted: {}", total_entities);
    println!("  Average entities per article: {:.2}", avg_entities);
    println!("{}", "=".repeat(80));

    Ok(())
}

fn print_geg(index: usize, geg: &GEGTable) {
    println!("{}", "━".repeat(80));
    println!("GEG Entry #{}", index);
    println!("{}", "━".repeat(80));

    println!("  Date: {}", geg.date);
    println!("  URL: {}", geg.url);
    println!("  Language: {}", geg.lang);

    // Sentiment scores
    println!("  📊 Sentiment:");
    println!("    Magnitude: {:.3}", geg.magnitude);
    if let Some(score) = geg.score {
        let sentiment = if score > 0.25 {
            "Positive"
        } else if score < -0.25 {
            "Negative"
        } else {
            "Neutral"
        };
        println!("    Score: {:.3} ({})", score, sentiment);
    }
    if let Some(polarity) = geg.polarity {
        println!("    Polarity: {:.3} [DEPRECATED]", polarity);
    }

    // Entities
    println!("  👥 Entities ({}):", geg.entities.len());

    if geg.entities.is_empty() {
        println!("    (No entities found)");
    } else {
        // Get top entities by salience
        let top_entities = geg.top_entities(10);

        for (i, entity) in top_entities.iter().enumerate() {
            println!(
                "    {}. {} ({:?})",
                i + 1,
                entity.name,
                entity.entity_type
            );
            println!(
                "       Mentions: {} | Salience: {:.3}",
                entity.num_mentions, entity.avg_salience
            );

            if let Some(mid) = &entity.mid {
                println!("       Google MID: {}", mid);
            }

            if let Some(wiki_url) = &entity.wikipedia_url {
                println!("       Wikipedia: {}", wiki_url);
            }
        }

        if geg.entities.len() > 10 {
            println!("    ... and {} more entities", geg.entities.len() - 10);
        }
    }

    // Entity statistics
    let stats = geg.entity_stats();
    println!("  📈 Entity Statistics:");
    println!("    Total entities: {}", stats.total_entities);
    println!("    With Google MID: {}", stats.entities_with_mid);
    println!("    With Wikipedia: {}", stats.entities_with_wikipedia);
    println!("    Total mentions: {}", stats.total_mentions);
    println!("    Average salience: {:.3}", stats.avg_salience);

    // Entity type breakdown
    use models::types::geg_table::GEGEntityType;
    let mut type_counts: std::collections::HashMap<GEGEntityType, usize> =
        std::collections::HashMap::new();

    for entity in &geg.entities {
        *type_counts.entry(entity.entity_type).or_insert(0) += 1;
    }

    if !type_counts.is_empty() {
        println!("  🏷️  Entity Types:");
        let mut sorted_types: Vec<_> = type_counts.iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(a.1));

        for (entity_type, count) in sorted_types.iter().take(5) {
            println!("    {:?}: {}", entity_type, count);
        }
    }

    println!();
}
