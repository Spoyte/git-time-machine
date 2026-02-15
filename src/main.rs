mod git;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "git-time-machine")]
#[command(about = "Visualize your repository's evolution through time")]
struct Args {
    /// Path to git repository (defaults to current directory)
    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let path = args.path.unwrap_or_else(|| PathBuf::from("."));
    let explorer = git::GitExplorer::open(&path)?;
    
    let commits = explorer.get_commit_history()?;
    
    println!("Git Time Machine - Found {} commits\n", commits.len());
    
    for commit in commits.iter().take(20) {
        let time = chrono::DateTime::from_timestamp(commit.time, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "???".to_string());
        
        println!("{}  {}  {}", 
            commit.short_id,
            time,
            commit.message
        );
    }
    
    if commits.len() > 20 {
        println!("\n... and {} more commits", commits.len() - 20);
    }
    
    Ok(())
}
