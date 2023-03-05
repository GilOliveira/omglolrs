use crate::structures::request_response::RequestResponse;
use crate::structures::weblog::WeblogConfigurationResponse;
use crate::structures::weblog::WeblogEntriesResponse;
use crate::structures::weblog::WeblogEntryResponse;
use crate::structures::weblog::WeblogTemplateResponse;

// TODO: Add bearer tokens when needed

pub async fn get_weblog_entries(
    address: &String,
) -> Result<RequestResponse<WeblogEntriesResponse>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!(
            "https://api.omg.lol/address/{address}/weblog/entries"
        ))
        .send()
        .await?
        .json::<RequestResponse<WeblogEntriesResponse>>()
        .await?;

    Ok(body)
}

pub async fn get_weblog_entry(
    address: &String,
    entry: &String,
) -> Result<RequestResponse<WeblogEntryResponse>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!(
            "https://api.omg.lol/address/{address}/weblog/entry/{entry}"
        ))
        .send()
        .await?
        .json::<RequestResponse<WeblogEntryResponse>>()
        .await?;

    Ok(body)
}

pub async fn get_latest_weblog_entry(
    address: &String,
) -> Result<RequestResponse<WeblogEntryResponse>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!(
            "https://api.omg.lol/address/{address}/weblog/post/latest"
        ))
        .send()
        .await?
        .json::<RequestResponse<WeblogEntryResponse>>()
        .await?;

    Ok(body)
}

pub async fn get_weblog_config(
    address: &String,
) -> Result<RequestResponse<WeblogConfigurationResponse>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!(
            "https://api.omg.lol/address/{address}/weblog/configuration"
        ))
        .send()
        .await?
        .json::<RequestResponse<WeblogConfigurationResponse>>()
        .await?;

    Ok(body)
}

pub async fn get_weblog_template(
    address: &String,
) -> Result<RequestResponse<WeblogTemplateResponse>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!(
            "https://api.omg.lol/address/{address}/weblog/template"
        ))
        .send()
        .await?
        .json::<RequestResponse<WeblogTemplateResponse>>()
        .await?;

    Ok(body)
}
