use super::{account::TimeStrings, request_response::RequestResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NowResponse {
    message: String,
    now: Vec<NowPage>,
}

#[derive(Deserialize, Debug)]
pub struct NowPage {
    content: String,
    updated: u32,
    listed: u8,
    nudge: u8,
    metadata: String,
}

#[derive(Deserialize, Debug)]
pub struct NowGardenResponse {
    message: String,
    garden: Vec<NowGarden>,
}

#[derive(Deserialize, Debug)]
pub struct NowGarden {
    address: String,
    url: String,
    updated: TimeStrings,
}
