pub struct DependantVariables {
    pub search_result_class_value : &'static str,
    pub  anime_title_tag: &'static str,

}

pub static  UTIL_VARS: &DependantVariables = &DependantVariables {
    search_result_class_value: "last_episodes",
    anime_title_tag: "name",
};