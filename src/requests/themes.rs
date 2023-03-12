use crate::structures::request_response::RequestResponse;
use crate::structures::themes::ProfileThemes;

pub async fn get_profile_themes(
) -> Result<RequestResponse<ProfileThemes>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://api.omg.lol/theme/list")
        .send()
        .await?
        .json::<RequestResponse<ProfileThemes>>()
        .await?;

    Ok(body)
}
