use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};
use clap::Parser;
use anyhow::{Context, Result};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "rgrep")]
#[command(about = "A small grep clone in Rust", version = "1.0.0", long_about = None)]
struct Args {
    pattern: String,
    path: PathBuf,
    
    #[arg(short = 'i', long)]
    ignore_case: bool,
    
    #[arg(short = 'n', long)]
    line_numbers: bool,
    
    #[arg(short = 'r', long)]
    recursive: bool,
    
    #[arg(short = 'c', long)]
    count: bool,
    
    #[arg(short = 'l', long)]
    files_with_matches: bool,
}

fn search_in_file(file_path: &PathBuf, pattern: &str, ignore_case: bool, line_numbers: bool, count_only: bool, files_with_matches: bool) -> Result<usize> {
    let file = File::open(file_path).with_context(| | format!("Failed to open file: {}", file_path.display()))?;
    
    let reader = BufReader::new(file);
    
    let pattern_mcp = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };
    
    let mut matches = 0usize;
    
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        let line_mcp = if ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };
        
        if line_mcp.contains(&pattern_mcp) {
            matches += 1;
            
            if files_with_matches {
                break;
            }
            
            if count_only {
                continue;
            }
            if line_numbers {
                println!("{}:{}:{}", file_path.display(), idx + 1, line);
            } else {
                println!("{}:{}", file_path.display(), line);
            }
        }
    }
    
    if files_with_matches && matches > 0 {
        println!("{}", file_path.display());
    }
    
    if count_only {
        println!("{}:{}", file_path.display(), matches);
    }
    
     Ok(matches)
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let mut total_matches = 0usize;
    
    let run_on_file = | file_path: PathBuf | -> Result<usize> {
        search_in_file(&args.path, &args.pattern, args.ignore_case, args.line_numbers, args.count, args.files_with_matches)
    };
    
    if args.path.is_file() {
        match run_on_file(args.path.clone()) {
            Ok(matches) => total_matches += matches,
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(2);
            },
        }
    } else if args.path.is_dir() {
        if !args.recursive {
            eprintln!(
                "Path is a directory. Use -r/--recursive to search recursively: {}",
                args.path.display()
            );
            std::process::exit(2);
        }
        
        for entry in WalkDir::new(&args.path).into_iter() {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("Error: {e}");
                    continue;
                }
            };
            
            if entry.file_type().is_file() {
                let file_path = entry.path().to_path_buf();
                match run_on_file(file_path) {
                    Ok(matches) => total_matches += matches,
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
    } else {
        anyhow::bail!("Invalid path: {}", args.path.display());
        std::process::exit(2);
    }
    
    if total_matches > 0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
