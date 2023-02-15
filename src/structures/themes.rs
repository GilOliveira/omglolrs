use std::collections::HashMap;
use serde::Deserialize;


// #[derive(Deserialize, Debug)]
// pub struct ThemeResponse {
//     request: RequestResponse,
//     response: ProfileThemes,
// }

#[derive(Deserialize, Debug)]
pub struct ProfileThemes {
    message: String,
    themes: HashMap<String, Theme>,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    id: String,
    name: String,
    created: String,
    updated: String,
    author: String,
    author_url: String,
    version: String,
    license: String,
    description: String,
    preview_css: String,
    sample_profile: String,
}
