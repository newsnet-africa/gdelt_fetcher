//! CSV Encoding Fix Utility
//!
//! This utility fixes UTF-8 encoding issues in the GCAM Master Codebook CSV file
//! by cleaning problematic characters and ensuring proper formatting.

use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() -> Result<()> {
    println!("=== CSV Encoding Fix Utility ===\n");

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() > 1 {
        &args[1]
    } else {
        "GCAM-MASTER-CODEBOOK.csv"
    };

    let output_path = if args.len() > 2 {
        &args[2]
    } else {
        "GCAM-MASTER-CODEBOOK-fixed.csv"
    };

    println!("Input file: {}", input_path);
    println!("Output file: {}", output_path);

    // Check if input file exists
    if !Path::new(input_path).exists() {
        eprintln!("❌ ERROR: Input file '{}' not found!", input_path);
        eprintln!("\nUsage: {} [input_csv] [output_csv]", args[0]);
        eprintln!("  input_csv:  Path to original CSV file (default: GCAM-MASTER-CODEBOOK.csv)");
        eprintln!(
            "  output_csv: Path to cleaned CSV file (default: GCAM-MASTER-CODEBOOK-fixed.csv)"
        );
        return Ok(());
    }

    println!("\n🔧 Processing CSV file...");

    let input_file = File::open(input_path)
        .with_context(|| format!("Failed to open input file '{}'", input_path))?;
    let reader = BufReader::new(input_file);

    let output_file = File::create(output_path)
        .with_context(|| format!("Failed to create output file '{}'", output_path))?;
    let mut writer = BufWriter::new(output_file);

    let mut line_count = 0;
    let mut cleaned_count = 0;
    let mut skipped_count = 0;

    for (line_num, line_result) in reader.lines().enumerate() {
        line_count += 1;

        match line_result {
            Ok(line) => {
                // Clean the line by replacing problematic characters
                let cleaned_line = clean_line(&line);

                // Check if the line was modified
                if cleaned_line != line {
                    cleaned_count += 1;
                    println!(
                        "  📝 Cleaned line {}: removed {} problematic characters",
                        line_num + 1,
                        line.len() - cleaned_line.len()
                    );
                }

                // Write the cleaned line
                writeln!(writer, "{}", cleaned_line)
                    .with_context(|| format!("Failed to write line {}", line_num + 1))?;
            }
            Err(e) => {
                // Handle UTF-8 errors by skipping the line
                eprintln!(
                    "⚠️  Skipping line {} due to encoding error: {}",
                    line_num + 1,
                    e
                );
                skipped_count += 1;
            }
        }

        // Progress indicator for large files
        if line_count % 1000 == 0 {
            println!("  📊 Processed {} lines...", line_count);
        }
    }

    writer
        .flush()
        .with_context(|| "Failed to flush output file")?;

    println!("\n✅ CSV cleaning completed!");
    println!("   📈 Total lines processed: {}", line_count);
    println!("   🧹 Lines cleaned: {}", cleaned_count);
    println!("   ⏭️  Lines skipped (encoding errors): {}", skipped_count);

    if skipped_count > 0 {
        println!(
            "\n⚠️  Warning: {} lines were skipped due to encoding errors.",
            skipped_count
        );
        println!("   These lines contained characters that couldn't be read as UTF-8.");
        println!("   The database population should now work with the remaining data.");
    }

    if cleaned_count > 0 {
        println!("\n🎯 Recommendation: Use the cleaned file for database population:");
        println!(
            "   cargo run --bin populate_gcam_db -- {} ./gcam_db",
            output_path
        );
    } else {
        println!("\n✨ No cleaning was needed - file was already properly encoded!");
        println!("   The original encoding error might be in a specific problematic line.");
        println!("   Try using the original file again, or check line 2614-2615 manually.");
    }

    Ok(())
}

/// Clean a single line by removing or replacing problematic characters
fn clean_line(line: &str) -> String {
    line.chars()
        .filter_map(|c| {
            match c {
                // Keep printable ASCII characters
                '\u{0020}'..='\u{007E}' => Some(c),
                // Keep common whitespace
                '\t' | '\n' | '\r' => Some(c),
                // Keep common extended ASCII that's usually safe
                '\u{00A0}'..='\u{00FF}' => {
                    // Replace some problematic characters with safe alternatives
                    match c {
                        '\u{00A0}' => Some(' '),               // Non-breaking space -> regular space
                        '\u{2013}' | '\u{2014}' => Some('-'),  // En/em dashes -> hyphen
                        '\u{2018}' | '\u{2019}' => Some('\''), // Smart quotes -> apostrophe
                        '\u{201C}' | '\u{201D}' => Some('"'),  // Smart quotes -> quote
                        _ => Some(c),
                    }
                }
                // Keep some common Unicode characters that are usually safe
                '\u{2013}' | '\u{2014}' => Some('-'), // En/em dashes
                '\u{2018}' | '\u{2019}' => Some('\''), // Smart single quotes
                '\u{201C}' | '\u{201D}' => Some('"'), // Smart double quotes
                '\u{2026}' => Some('.'),              // Ellipsis -> period
                // Remove everything else that might cause problems
                _ => {
                    if c.is_control() && c != '\t' && c != '\n' && c != '\r' {
                        None // Remove control characters
                    } else if c as u32 > 0xFFFF {
                        None // Remove high Unicode characters that might cause issues
                    } else {
                        Some(c) // Keep other characters
                    }
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_line() {
        // Test basic ASCII - should remain unchanged
        assert_eq!(clean_line("hello world"), "hello world");

        // Test tab-delimited data - should remain unchanged
        assert_eq!(clean_line("c1.1\t1\t1\tWORDCOUNT"), "c1.1\t1\t1\tWORDCOUNT");

        // Test smart quotes - should be replaced
        assert_eq!(
            clean_line("word \u{201C}quote\u{201D} word"),
            "word \"quote\" word"
        );

        // Test non-breaking space - should be replaced with regular space
        assert_eq!(clean_line("word\u{00A0}word"), "word word");

        // Test control characters - should be removed
        assert_eq!(clean_line("word\u{0001}word"), "wordword");
    }
}
