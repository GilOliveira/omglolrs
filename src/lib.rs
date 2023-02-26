pub mod structures;
pub mod requests;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn main() {
        // let info = get_statuslog_status("gil", "63a8a6b3cbdc5").await;
        // let info = get_all_statuslog_statuses("gil").await;

        let info = requests::themes::get_profile_themes().await;

        println!("response = {:#?}", info);
    }
}

