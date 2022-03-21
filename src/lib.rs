use anyhow::{Context, Result};
use select::document::Document;
use select::predicate::{Attr, Class, Predicate};
use types::AnimeCollection;
use util::UTIL_VARS;

pub mod types;
pub mod util;

pub fn search_anime(keyword: &str) -> Result<AnimeCollection> {
    let search_url = format!("{}//search.html?keyword={}", UTIL_VARS.base_url, keyword);
    let body = reqwest::blocking::get(search_url)?.text()?;

    let html = body.to_string();
    let document = Document::from_read(html.as_bytes())?;

    // print_elements(&document, &util::UTIL_VARS, std::io::stdout()).with_context(|| "error priting matching anime titles")?;
    let elements = document
        .find(
            Class(util::UTIL_VARS.search_result_class_value)
                .descendant(Attr("class", util::UTIL_VARS.anime_title_tag)),
        )
        .map(|node| {
            let anime_node = node.first_child().unwrap();
            types::Anime {
                title: anime_node.attr("title").unwrap().to_string(),
                url: format!(
                    "{}{}",
                    UTIL_VARS.base_url.to_string(),
                    anime_node.attr("href").unwrap().to_string()
                ),
                episodes: Vec::new(),
            }
        })
        .collect();

    Ok(elements)
}
