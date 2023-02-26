use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PasteResponse {
    message: String,
    paste: Paste,
}

#[derive(Deserialize, Debug)]
pub struct PastebinResponse {
    message: String,
    success: Vec<Paste>,
}

#[derive(Deserialize, Debug)]
pub struct Paste {
    title: String,
    content: String,
    modified_on: String,
}
