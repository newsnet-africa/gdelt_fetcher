use anyhow::Result;
use gdelt_fetcher::{fetch_and_parse_latest_gkg, GKGTable};
use std::env;

/// Fetch and display GDELT Global Knowledge Graph (GKG) table entries
///
/// Usage: cargo run --example fetch_gkg [count]
///
/// Arguments:
///   count - Number of items to display (default: 5)
///
/// Example:
///   cargo run --example fetch_gkg 10
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Get count from command line args, default to 5 (GKG entries are verbose)
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(5)
    } else {
        5
    };

    println!("{}", "=".repeat(80));
    println!("GDELT Global Knowledge Graph (GKG) Fetcher");
    println!("{}", "=".repeat(80));
    println!("Fetching latest GDELT GKG data...\n");

    let gkg_entries = fetch_and_parse_latest_gkg().await?;

    println!("✅ Successfully fetched {} GKG entries", gkg_entries.len());
    println!("📊 Displaying first {} entries:\n", count);

    for (i, gkg) in gkg_entries.iter().take(count).enumerate() {
        print_gkg(i + 1, gkg);
    }

    println!("\n{}", "=".repeat(80));
    println!("Summary:");
    println!("  Total GKG entries fetched: {}", gkg_entries.len());
    println!("  Entries displayed: {}", count.min(gkg_entries.len()));
    println!("{}", "=".repeat(80));

    Ok(())
}

fn print_gkg(index: usize, gkg: &GKGTable) {
    println!("{}", "━".repeat(80));
    println!("GKG Entry #{}", index);
    println!("{}", "━".repeat(80));

    println!("  GKGRECORD ID: {} (Seq: {})", gkg.global_knowledge_graph_id.record_date, gkg.global_knowledge_graph_id.sequence);
    println!("  Date: {}", gkg.date);
    println!("  Source Collection ID: {:?}", gkg.source_collection_identifier);
    println!("  Source: {}", gkg.source_common_name);
    println!("  Document: {}", gkg.document_identifier);

    if gkg.document_identifier.starts_with("http") {
        println!("    Type: URL/Web Article");
    }

    // V2 Enhanced Themes
    if !gkg.v2_enhanced_themes.is_empty() {
        println!("  📌 Themes ({}):", gkg.v2_enhanced_themes.len());
        for (i, theme) in gkg.v2_enhanced_themes.iter().take(5).enumerate() {
            println!("    {}. {}", i + 1, theme.name);
        }
        if gkg.v2_enhanced_themes.len() > 5 {
            println!("    ... and {} more themes", gkg.v2_enhanced_themes.len() - 5);
        }
    }

    // V2 Enhanced Locations
    if !gkg.v2_enhanced_locations.is_empty() {
        println!("  📍 Locations ({}):", gkg.v2_enhanced_locations.len());
        for (i, (location, _offset)) in gkg.v2_enhanced_locations.iter().take(5).enumerate() {
            if let Some(fullname) = &location.fullname {
                println!("    {}. {}", i + 1, fullname);
            }
        }
        if gkg.v2_enhanced_locations.len() > 5 {
            println!("    ... and {} more locations", gkg.v2_enhanced_locations.len() - 5);
        }
    }

    // V2 Enhanced Persons
    if !gkg.v2_enhanced_persons.is_empty() {
        println!("  👤 Persons ({}):", gkg.v2_enhanced_persons.len());
        for (i, person) in gkg.v2_enhanced_persons.iter().take(5).enumerate() {
            println!("    {}. {}", i + 1, person.name);
        }
        if gkg.v2_enhanced_persons.len() > 5 {
            println!("    ... and {} more persons", gkg.v2_enhanced_persons.len() - 5);
        }
    }

    // V2 Enhanced Organizations
    if !gkg.v2_enhanced_organizations.is_empty() {
        println!("  🏢 Organizations ({}):", gkg.v2_enhanced_organizations.len());
        for (i, org) in gkg.v2_enhanced_organizations.iter().take(5).enumerate() {
            println!("    {}. {}", i + 1, org.name);
        }
        if gkg.v2_enhanced_organizations.len() > 5 {
            println!("    ... and {} more organizations", gkg.v2_enhanced_organizations.len() - 5);
        }
    }

    // Tone
    println!("  🎵 Tone: {:.2} (Score: {:.2})", gkg.tone.tone, gkg.tone.positive_score - gkg.tone.negative_score);

    // GCAM data
    if !gkg.gcam.is_empty() {
        println!("  📊 GCAM Measures ({}):", gkg.gcam.len());
        for (i, entry) in gkg.gcam.iter().take(5).enumerate() {
            println!("    {}. {} = {:.2}", i + 1, entry.key, entry.value);

            // If metadata is available, show it
            if let Some(metadata) = &entry.metadata {
                println!("       Dictionary: {:?} | Dimension: {}", metadata.dictionary, metadata.dimension_name);
            }
        }
        if gkg.gcam.len() > 5 {
            println!("    ... and {} more GCAM measures", gkg.gcam.len() - 5);
        }
    }

    // Enhanced dates
    if !gkg.enhanced_dates.is_empty() {
        println!("  📅 Enhanced Dates ({}):", gkg.enhanced_dates.len());
        for (i, date) in gkg.enhanced_dates.iter().take(3).enumerate() {
            println!("    {}. {:?}", i + 1, date);
        }
        if gkg.enhanced_dates.len() > 3 {
            println!("    ... and {} more dates", gkg.enhanced_dates.len() - 3);
        }
    }

    // Social media sharing data
    if let Some(sharing) = &gkg.sharing_image {
        println!("  🔗 Sharing Image: {}", sharing);
    }

    // Related images
    if !gkg.related_images.is_empty() {
        println!("  🖼️  Related Images ({}):", gkg.related_images.len());
        for (i, image) in gkg.related_images.iter().take(2).enumerate() {
            println!("    {}. {}", i + 1, image);
        }
        if gkg.related_images.len() > 2 {
            println!("    ... and {} more images", gkg.related_images.len() - 2);
        }
    }

    // Social media embeds
    if !gkg.social_image_embeds.is_empty() {
        println!("  📱 Social Media Image Embeds ({}):", gkg.social_image_embeds.len());
        for (i, embed) in gkg.social_image_embeds.iter().take(2).enumerate() {
            println!("    {}. {}", i + 1, embed);
        }
        if gkg.social_image_embeds.len() > 2 {
            println!("    ... and {} more embeds", gkg.social_image_embeds.len() - 2);
        }
    }

    println!();
}
