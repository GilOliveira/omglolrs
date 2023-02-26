use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PurlsResponse {
    message: String,
    purls: Vec<Purl>,
}

#[derive(Deserialize, Debug)]
pub struct Purl {
    name: String,
    url: String,
    counter: Option<i32>,
}
