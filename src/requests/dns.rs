use crate::structures::dns::DNSrecords;
use crate::structures::message_response::MessageResponse;
use crate::structures::request_response::RequestResponse;

/// Get all DNS records for an address
///
/// # Arguments
/// * `address` - The address for which you want to query DNS records
/// * `api_key` - An API key with permission to access records for the `address`
pub async fn get_dns_records(
    address: &String,
    api_key: &String,
) -> Result<RequestResponse<DNSrecords>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://api.omg.lol/address/{address}/dns"))
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await?
        .json::<RequestResponse<DNSrecords>>()
        .await?;

    Ok(body)
}

/// Delete a DNS record
///
/// # Arguments
/// * `address` - The address with which the DNS record is associated
/// * `id` - The unique ID of the DNS record
/// * `api_key` - An API key with permission to access records for the `address`
pub async fn delete_dns_record(
    address: &String,
    id: &String,
    api_key: &String,
) -> Result<MessageResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .delete(format!("https://api.omg.lol/address/{address}/dns/{id}"))
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await?
        .json::<MessageResponse>()
        .await?;

    Ok(body)
}
