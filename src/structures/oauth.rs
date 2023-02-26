use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Token {
    access_token: String,
    token_type: String,
    scope: String,
}
