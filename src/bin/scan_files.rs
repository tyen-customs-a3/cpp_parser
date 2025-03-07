use cpp_parser::parse;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::Mutex;
use clap::Parser;
use log::{info, warn, error, debug, LevelFilter};
use env_logger::Builder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to scan
    #[arg(short, long, default_value = ".")]
    directory: String,
    
    /// File extensions to scan (comma-separated)
    #[arg(short, long, default_value = "cpp,hpp,h")]
    extensions: String,
    
    /// Batch size for parallel processing
    #[arg(short, long, default_value = "100")]
    batch_size: usize,
    
    /// Follow symbolic links
    #[arg(short, long)]
    follow_links: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    log_level: String,
    
    /// Show context around errors (number of lines)
    #[arg(short, long, default_value = "3")]
    context_lines: usize,
}

/// Extract context lines around an error
fn extract_context(content: &str, line: usize, context_lines: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    
    let start_line = if line > context_lines { line - context_lines } else { 1 };
    let end_line = std::cmp::min(line + context_lines, total_lines);
    
    let mut context = String::new();
    context.push_str(&format!("Context (lines {}-{}):\n", start_line, end_line));
    
    for i in start_line..=end_line {
        if i <= total_lines {
            let line_marker = if i == line { ">" } else { " " };
            context.push_str(&format!("{} {:4}: {}\n", line_marker, i, lines[i-1]));
        }
    }
    
    context
}

fn main() {
    // Initialize the logger with appropriate level based on arguments
    let args = Args::parse();
    
    let mut builder = Builder::new();
    
    // Set log level based on command line argument or verbose flag
    let level = match args.log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => {
            if args.verbose {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            }
        }
    };
    
    builder.filter_level(level);
    builder.init();
    
    info!("Log level set to: {}", level);
    
    // Convert extensions string to Vec<String>
    let extensions: Vec<String> = args.extensions
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    
    // Collect all C++ files
    let files: Vec<_> = WalkDir::new(&args.directory)
        .follow_links(args.follow_links)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                extensions.contains(&ext.to_string_lossy().to_string())
            } else {
                false
            }
        })
        .map(|e| e.path().to_path_buf())
        .collect();
    
    info!("Found {} files to process", files.len());
    
    // Process files in batches
    let failed_files = Mutex::new(Vec::new());
    let total_files = files.len();
    
    files.par_chunks(args.batch_size)
        .enumerate()
        .for_each(|(batch_idx, batch)| {
            let start_idx = batch_idx * args.batch_size;
            let end_idx = (start_idx + batch.len()).min(total_files);
            
            info!("Processing batch {}/{} (files {}-{})", 
                batch_idx + 1, 
                (total_files + args.batch_size - 1) / args.batch_size,
                start_idx + 1,
                end_idx
            );
            
            for file in batch {
                match fs::read_to_string(file) {
                    Ok(content) => {
                        match parse(&content) {
                            Ok(_) => {
                                if args.verbose {
                                    info!("Successfully parsed {}", file.display());
                                }
                            }
                            Err(e) => {
                                // Extract line number from error message if available
                                let error_msg = e.to_string();
                                let line_number = if error_msg.contains("line") {
                                    error_msg.split("line")
                                        .nth(1)
                                        .and_then(|s| s.split(',').next())
                                        .and_then(|s| s.trim().parse::<usize>().ok())
                                        .unwrap_or(1)
                                } else {
                                    1
                                };
                                
                                // Get context around the error
                                let context = extract_context(&content, line_number, args.context_lines);
                                
                                error!("Failed to parse {}:\n{}\n{}", file.display(), e, context);
                                failed_files.lock().unwrap().push(file.clone());
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to read {}: {}", file.display(), e);
                        failed_files.lock().unwrap().push(file.clone());
                    }
                }
            }
        });
    
    let failed = failed_files.lock().unwrap();
    if !failed.is_empty() {
        warn!("{} files failed to parse:", failed.len());
        std::process::exit(1);
    }
    
    info!("All files processed successfully");
} 