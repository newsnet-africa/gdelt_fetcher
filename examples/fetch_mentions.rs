use anyhow::Result;
use gdelt_fetcher::{fetch_and_parse_latest_mentions, MentionTable};
use std::env;

/// Fetch and display GDELT Mention table entries
///
/// Usage: cargo run --example fetch_mentions [count]
///
/// Arguments:
///   count - Number of items to display (default: 10)
///
/// Example:
///   cargo run --example fetch_mentions 15
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Get count from command line args, default to 10
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };

    println!("{}", "=".repeat(80));
    println!("GDELT Mention Table Fetcher");
    println!("{}", "=".repeat(80));
    println!("Fetching latest GDELT mentions...\n");

    let mentions = fetch_and_parse_latest_mentions().await?;

    println!("✅ Successfully fetched {} mentions", mentions.len());
    println!("📊 Displaying first {} mentions:\n", count);

    for (i, mention) in mentions.iter().take(count).enumerate() {
        print_mention(i + 1, mention);
    }

    println!("\n{}", "=".repeat(80));
    println!("Summary:");
    println!("  Total mentions fetched: {}", mentions.len());
    println!("  Mentions displayed: {}", count.min(mentions.len()));
    println!("{}", "=".repeat(80));

    Ok(())
}

fn print_mention(index: usize, mention: &MentionTable) {
    println!("{}", "━".repeat(80));
    println!("Mention #{}", index);
    println!("{}", "━".repeat(80));

    println!("  Global Event ID: {}", mention.global_event_id.0);
    println!("  Event Date: {}", mention.event_date);
    println!("  Mention Date: {}", mention.mention_date);

    // Mention type
    println!("  Mention Type: {:?}", mention.mention_type);

    // Source information
    println!("  Source: {}", mention.mention_source_name.0);

    // Sentence ID
    println!("  Sentence ID: {}", mention.sentence_id.0);

    // Character offsets
    if let Some(offset) = &mention.actor_1_char_offset {
        println!("  Actor 1 Character Offset: {}", offset.0);
    }
    if let Some(offset) = &mention.actor_2_char_offset {
        println!("  Actor 2 Character Offset: {}", offset.0);
    }
    if let Some(offset) = &mention.action_char_offset {
        println!("  Action Character Offset: {}", offset.0);
    }

    // In raw text
    println!("  In Raw Text: {}", mention.in_raw_text.0);

    // Confidence
    println!("  Confidence: {}%", mention.confidence.0);

    // Document length
    println!("  Document Length: {} chars", mention.mention_doc_len.0);

    // Document tone
    let doc_tone = mention.mention_doc_tone.0;
    let tone_description = if doc_tone > 0.0 {
        "Positive"
    } else if doc_tone < 0.0 {
        "Negative"
    } else {
        "Neutral"
    };
    println!("  Document Tone: {:.2} ({})", doc_tone, tone_description);

    // Translation information
    let (source_lang, engine) = &mention.mention_doc_translation_info;
    if source_lang.is_some() || engine.is_some() {
        print!("  Translation:");
        if let Some(lang) = source_lang {
            let lang_str = std::str::from_utf8(&lang.0).unwrap_or("???");
            print!(" Language={}", lang_str);
        }
        if let Some(eng) = engine {
            print!(" Engine={}", eng.0);
        }
        println!();
    }

    // Extras (additional metadata)
    if !mention.extras.is_empty() {
        println!("  Extras: {}", mention.extras);
    }

    println!();
}
