use anyhow::{Result, Context};
use select::document::Document;
use select::predicate::{Class, Attr, Predicate};

pub mod util;

pub fn print_elements(document: &Document, util_vars: &util::DependantVariables,  mut writer: impl std::io::Write) -> Result<()> {
    let mut i = 1;

    let nodes = document.find(Class(util_vars.search_result_class_value).descendant(Attr("class", util_vars.anime_title_tag)));

    // let titles: Vec<_> = nodes.into_iter().map(|node| (node.text(), node.attr("href").unwrap().to_string())).collect();


    for node in   nodes {
        if node.text().is_empty() {
            continue;
        }

        writeln!(writer, "{}. {}", i, node.text()).with_context(|| "error while writing related titles")?;
        i+=1;
    }

    Ok(())
}


pub fn search_anime(keyword: &str) -> Result<()> {
    let search_url = format!("https://gogoanime.fi//search.html?keyword={}", keyword);
    let body = reqwest::blocking::get(search_url)?
        .text()?;

    let html = body.to_string();
    let document = Document::from_read(html.as_bytes())?;

    print_elements(&document, &util::UTIL_VARS, std::io::stdout()).with_context(|| "error priting matching anime titles")?;

    Ok(())
}