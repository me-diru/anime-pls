pub struct DependantVariables {
    pub base_url: &'static str,
    pub search_result_class_value: &'static str,
    pub anime_title_tag: &'static str,
    pub episode_list_active: &'static str,
    pub latest_episode_value: &'static str,
}

pub static UTIL_VARS: &DependantVariables = &DependantVariables {
    base_url: "https://gogoanime.fi",
    search_result_class_value: "last_episodes",
    anime_title_tag: "name",
    episode_list_active: "active",
    latest_episode_value: "ep_end",
};
