use anyhow::Result;
use select::document::Document;
use select::predicate::{Attr, Class, Predicate};
use types::AnimeCollection;
use util::UTIL_VARS;

pub mod types;
pub mod util;

fn get_document(url: &String) -> Result<Document> {
    let resp = reqwest::blocking::get(url)?;
    let body = resp.text()?;
    let html = body.to_string();
    let document = Document::from_read(html.as_bytes())?;
    Ok(document)
}

pub fn search_anime(keyword: &str) -> Result<AnimeCollection> {
    let search_url = format!("{}//search.html?keyword={}", UTIL_VARS.base_url, keyword);

    let document = get_document(&search_url)?;

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

pub fn get_episodes(anime: &types::Anime) -> Result<Vec<u32>> {
    let document = get_document(&anime.url.clone())?;

    let mut episodes: Vec<u32> = Vec::new();
    episodes.push(1);

    let active_episodes = document
        .find(Class(util::UTIL_VARS.episode_list_active))
        .last()
        .unwrap();
    let latest_episode = active_episodes
        .attr(util::UTIL_VARS.latest_episode_value)
        .unwrap();
    episodes.push(latest_episode.parse::<u32>().unwrap());

    Ok(episodes)
}

pub fn get_episode_link(anime: &types::Anime, episode: String) -> Result<String> {
    let mut url = anime.url.clone();
    url = url.split("/").last().unwrap().to_string();
    url = format!("{}/{}-episode-{}", UTIL_VARS.base_url, url, episode);

    Ok(url)
}
