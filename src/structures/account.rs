use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AcccountResponse {
    message: String,
    email: String,
    name: String,
    api_key: String,
    created: TimeStrings,
    settings: AccountSettings,
}

#[derive(Deserialize, Debug)]
pub struct TimeStrings {
    unix_epoch_time: String,
    iso_8601_time: String,
    rfc_2822_time: String,
    relative_time: String,
}

#[derive(Deserialize, Debug)]
pub struct AccountSettings {
    owner: String,
    communication: Option<String>, // TODO: Remove "Option" if patched upstream
    date_format: Option<String>,   // https://github.com/neatnik/omg.lol/issues/613
    web_editor: String,
}
