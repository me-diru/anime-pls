pub struct DependantVariables {
    pub base_url: &'static str,
    pub search_result_class_value: &'static str,
    pub anime_title_tag: &'static str,
    pub active_tag: &'static str,
    pub latest_episode_value: &'static str,
    pub video_embed_tag: &'static str,
    pub secret_key: &'static str,
    pub iv: &'static str,
    pub video_host: &'static str,
}

pub static UTIL_VARS: &DependantVariables = &DependantVariables {
    base_url: "https://gogoanime.fi",
    search_result_class_value: "last_episodes",
    anime_title_tag: "name",
    active_tag: "active",
    latest_episode_value: "ep_end",
    video_embed_tag: "data-video",
    secret_key: "3235373136353338353232393338333936313634363632323738383333323838",
    iv: "31323835363732393835323338333933",
    video_host: "https://gogoplay4.com/encrypt-ajax.php",
};
