pub mod structures;
pub mod requests;

use requests::themes::get_profile_themes;

#[tokio::main]
async fn main() {
    // let info = get_statuslog_status("gil", "63a8a6b3cbdc5").await;
    // let info = get_all_statuslog_statuses("gil").await;

    let info = get_profile_themes().await;

    println!("response = {:#?}", info);
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
