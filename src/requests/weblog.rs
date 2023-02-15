use super::super::structures::weblog::WeblogResponse;

pub async fn get_weblog_entries(address: &str) -> Result<WeblogEntriesResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://api.omg.lol/address/{address}/weblog/entries"))
        .send()
        .await?
        .json::<WeblogResponse>()
        .await?;

    Ok(body)
}

pub async fn get_weblog_entry(address: &str, entry: &str) -> Result<WeblogEntriesResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://api.omg.lol/address/{address}/weblog/entry/{entry}"))
        .send()
        .await?
        .json::<WeblogResponse>()
        .await?;

    Ok(body)
}


pub async fn get_latest_weblog_entry(address: &str) -> Result<WeblogEntriesResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://api.omg.lol/address/{address}/weblog/post/latest"))
        .send()
        .await?
        .json::<WeblogResponse>()
        .await?;

    Ok(body)
}
