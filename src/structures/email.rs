use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ForwardingAddresses {
    message: String,
    destination_string: String,
    destination_array: Vec<String>,
    address: String,
    email_address: String,
}
