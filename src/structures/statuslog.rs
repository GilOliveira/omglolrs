use serde::Deserialize;

use super::request_response::RequestResponse;

#[derive(Deserialize, Debug)]
pub struct StatuslogResponse {
    request: RequestResponse,
    response: StatuslogResponseArray,
}

#[derive(Deserialize, Debug)]
pub struct StatuslogResponseArray {
    message: String,
    status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    id: String,
    address: String,
    created: String,
    relative_time: String,
    emoji: char,
    content: String,
    external_url: Option<String>,
}
