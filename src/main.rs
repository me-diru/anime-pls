
use ani_cli::{search_anime};
use anyhow::{Result};
use clap::{Parser};

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    keyword: String,
}




fn main() -> Result<()>{

    let args = Cli::parse();
    
    search_anime(&args.keyword)?;

    Ok(())
}
