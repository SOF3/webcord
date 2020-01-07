use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref EMOJI_LIST: HashMap<String, String> = {
        reqwest::blocking::Client::new()
            .get("https://api.github.com/emojis")
            .send()
            .expect("Failed to fetch emoji list from GitHub API")
            .json::<HashMap<String, String>>()
            .expect("Failed to parse emoji list from GitHub API")
    };
}
