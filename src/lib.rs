use anyhow::Result;
use argon2::{self, Config};
use openssl::aes::{aes_ige, AesKey};
use openssl::symm::Mode;

use select::document::Document;
use select::predicate::{Attr, Class, Predicate};
pub mod types;

use types::AnimeCollection;
use util::UTIL_VARS;

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
        .find(Class(util::UTIL_VARS.active_tag))
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
pub fn get_video_link(episode_url: &String) -> Result<String> {
    let document = get_document(episode_url).unwrap();
    let video_tag = document
        .find(Class(util::UTIL_VARS.embed_video_tag))
        .last()
        .unwrap()
        .children()
        .nth(1)
        .unwrap();

    let video_embed = video_tag.attr(util::UTIL_VARS.video_embed_tag).unwrap();
    let video_link = format!("{}{}", "https:", video_embed);
    Ok(video_link)
}

// not able to use this way :/
pub fn decrypt_link(video_link: &String) -> Result<String> {
    let mut id = video_link.split("=").nth(1).unwrap().to_string();
    id = id.split("&").nth(0).unwrap().to_string();
    println!("got id from video link: {}", id);
    if id.len() < 8 {
        let padding = "10160310	0304	"; // hacky way to pad the id

        let mut iter = padding.chars();
        iter.by_ref().nth(8 - (id.len() % 16));
        let slice = iter.as_str();
        id = format!("{}{}", slice, id);
    } else {
        let last_char = id.chars().last().unwrap();
        println!("last char: {}", last_char);
        let mut digit_char = last_char.to_digit(10).unwrap();
        digit_char = digit_char ^ 10;
        let octal_char = format!("{:o}", digit_char);
        id = id.replace(last_char, &octal_char);
    }
    let padding_2 = "10160310	0304	"; // hacky way to pad the id

    let final_id = format!("{}{}", id, padding_2);
    let mut config = Config::default();
    config.hash_length = 128;

    let hash = argon2::hash_raw(
        final_id.as_bytes(),
        UTIL_VARS.secret_key.as_bytes(),
        &config,
    )?;

    let key = AesKey::new_encrypt(&hash[..]).unwrap();
    // let iv_bytes = UTIL_VARS.iv.as_bytes();
    let mut temp_iv = UTIL_VARS.iv.clone().to_string();

    unsafe {
        let mut iv = temp_iv.as_bytes_mut();
        let mut output = [0u8; 16];
        aes_ige(
            final_id.as_bytes(),
            &mut output,
            &key,
            &mut iv,
            Mode::Encrypt,
        );
        let encrypted_id = String::from_utf8(output.to_vec()).unwrap();

        let client = reqwest::blocking::Client::new();
        // :
        let response = client
            .get(UTIL_VARS.video_host)
            .header("X-Requested-With", "XMLHttpRequest")
            .body(format!("id={}", encrypted_id))
            .send()?;

        println!("{:?}", response);
    }

    Ok(id)
}
