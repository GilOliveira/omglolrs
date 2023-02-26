use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WebResponse {
    message: String,
    content: String,
    css: String,
    head: String,
    verified: String,
    pfp: String,
    metadata: String,
    branding: String,

    #[serde(rename(deserialize = "type"))]
    page_type: String,
}
