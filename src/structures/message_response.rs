use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}
