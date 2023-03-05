use super::super::structures::request_response::RequestResponse;
use super::super::structures::statuslog::StatuslogResponseArray;

pub async fn get_all_statuslog_statuses(
    address: &str,
) -> Result<RequestResponse<StatuslogResponseArray>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://api.omg.lol/address/{address}/statuses"))
        .send()
        .await?
        .json::<RequestResponse<StatuslogResponseArray>>()
        .await?;

    Ok(body)
}

pub async fn get_statuslog_status(
    address: &String,
    id: &String,
) -> Result<RequestResponse<StatuslogResponseArray>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!(
            "https://api.omg.lol/address/{address}/statuses/{id}"
        ))
        .send()
        .await?
        .json::<RequestResponse<StatuslogResponseArray>>()
        .await?;

    Ok(body)
}
