pub mod requests;
pub mod structures;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    use email_address::*;
    use std::str::FromStr;
    // These tests are just for debugging, no proper test-driven development here
    // don't use this for anything

    #[tokio::test]
    async fn main() {
        // If you're learning Rust best practices, maybe you should ignore the next
        // 2 lines. This is a test function so does anyone *actually* care?
        let address = env::var("OMGLOL_ADDRESS").unwrap_or("foobar".to_string());
        let api_key = env::var("OMGLOL_API_KEY").unwrap_or("".to_string());
        println!("Using account {} for testing", &address);
        //let info = requests::dns::get_dns_records(&address, &api_key).await;
        // let info = requests::themes::get_profile_themes().await;
        // let info = get_statuslog_status(&address, "63a8a6b3cbdc5").await;
        // let info = get_all_statuslog_statuses(&address).await;
        let destinations = vec![
            EmailAddress::from_str("foo@example.net").unwrap(),
            EmailAddress::from_str("bar@example.net").unwrap(),
        ];
        let info =
            requests::email::set_forwarding_addresses(&address, &destinations, &api_key).await;
        println!("response = {:#?}", info);
    }
}
