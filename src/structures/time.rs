use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TimeStrings {
    unix_epoch_time: String,
    iso_8601_time: String,
    rfc_2822_time: String,
    relative_time: String,
}
