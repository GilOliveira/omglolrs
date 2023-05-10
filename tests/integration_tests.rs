use omglol::{
    client::{Auth, NoAuth},
    structures::{DNStype, RequestError},
    OmglolClient,
};

use dotenv::dotenv;
use std::env;

fn init_noauth_client() -> (OmglolClient<NoAuth>, String) {
    dotenv().ok();
    let address = env::var("OMGLOL_ADDRESS").unwrap_or("foobar".to_string());
    let client_noauth = OmglolClient::new();
    println!("Using account {} for testing", &address);

    return (client_noauth, address);
}

fn init_auth_client() -> (OmglolClient<Auth>, String) {
    dotenv().ok();
    let address = env::var("OMGLOL_ADDRESS").unwrap_or("foobar".to_string());
    let api_key = env::var("OMGLOL_API_KEY").unwrap_or("".to_string());
    let client = OmglolClient::new().auth(api_key);
    println!("Using account {} for testing", &address);

    return (client, address);
}

#[tokio::test]
async fn get_all_statuses() {
    let (client, address) = init_noauth_client();
    let response = client.get_all_statuses(&address).await.unwrap().response;
    println!("{:#?}", response);
}

#[test]
fn dns_display() {
    assert!(format!("{}", DNStype::A) == "A");
}

#[tokio::test]
async fn get_service_status() {
    let (client, _) = init_noauth_client();
    let service_status = client.service_status().await.unwrap().response;
    println!("{:#?}", service_status);
}

#[tokio::test]
async fn get_profile_themes() {
    let (client, _) = init_noauth_client();
    let info = client.get_profile_themes().await.unwrap().response;
    println!("{:#?}", info);
}

#[tokio::test]
async fn get_dns_records() {
    let (client, address) = init_auth_client();
    let response = client.get_dns_records(&address).await.unwrap().response;
    println!("{:#?}", response);
}

#[tokio::test]
async fn get_web_page() {
    let (client, address) = init_auth_client();
    let response = client.get_web_page(&address).await.unwrap().response;
    println!("{:#?}", response);
}

// #[tokio::test]
// async fn query_unauthorized_endpoint() {
//     let (client, _) = init_auth_client();
//     let response_result = client.get_web_page("foobar")
//                               .await;
//     let raised_error = *&dyn response_result.err().unwrap();
//     assert_eq!(*raised_error.status_code, 401);
//     println!("{:#?}", response_result);
// }
