use serde::Deserialize;

use super::request_response::RequestResponse;

#[derive(Deserialize, Debug)]
pub struct ServiceStatusResponse {
    request: RequestResponse,
    response: ServiceStatus,
}

#[derive(Deserialize, Debug)]
pub struct ServiceStatus {
    message: String,
    members: i32,
    addresses: i32,
    profiles: i32,
}
