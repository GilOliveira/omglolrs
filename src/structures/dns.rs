use super::request_response::RequestResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DNSrecords {
    message: String,
    dns: Vec<DNSrecord>,
}

#[derive(Deserialize, Debug)]
pub struct DNSrecord {
    id: i32,
    name: String,
    data: String,
    priority: Option<i32>,
    ttl: i32,
    created_at: Option<String>,
    updated_at: Option<String>,

    #[serde(rename = "type")]
    record_type: DNStype, // record_type: String,
}

#[derive(Deserialize, Debug)]
pub enum DNStype {
    A,
    AAA,
    CAA,
    CNAME,
    MX,
    NS,
    SRV,
    TXT,
}
