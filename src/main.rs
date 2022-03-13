
use ani_cli::util;
use reqwest;
use anyhow::{Result, Context};
use select::document::Document;
use clap::{Parser};

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    keyword: String,
}




fn main() -> Result<()>{

    let args = Cli::parse();
    let search_url = format!("https://gogoanime.fi//search.html?keyword={}", args.keyword);
    
    

    let body = reqwest::blocking::get(search_url)?
    .text()?;

    let html = body.to_string();
    let document = Document::from_read(html.as_bytes())?;

  
    ani_cli::print_elements(&document, &util::UTIL_VARS, std::io::stdout()).with_context(|| "error priting matching anime titles")?;

    Ok(())
}
