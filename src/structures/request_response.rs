use serde::Deserialize;
// use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct RequestResponse<T> {
    request: RequestStatus,
    response: T,
}

#[derive(Deserialize, Debug)]
pub struct RequestStatus {
    status_code: u16,
    success: bool,
}
