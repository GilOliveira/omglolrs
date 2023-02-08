use serde::Deserialize;

pub mod structures;

use structures::service_status::ServiceStatusResponse;
use structures::statuslog::StatuslogResponse;

#[tokio::main]
async fn main() {
    let info = get_statuslog_status("gil", "63a8a6b3cbdc5").await;

    println!("response = {:#?}", info);
}

async fn get_statuslog_status(
    address: &str,
    id: &str,
) -> Result<StatuslogResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!(
            "https://api.omg.lol/address/{address}/statuses/{id}"
        ))
        .send()
        .await?
        .json::<StatuslogResponse>()
        .await?;

    Ok(body)
}

// async fn get_omglol_status() -> Result<ServiceStatusResponse, Box<dyn std::error::Error>> {
//     let client = reqwest::Client::new();
//     let body = client
//         .get("https://api.omg.lol/service/info")
//         .send()
//         .await?
//         .json::<ServiceStatusResponse>()
//         .await?;
//
//     Ok(body)
// }
