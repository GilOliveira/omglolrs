// SPDX-LICENSE-IDENTIFIER: MPL-2.0

/*
omglol crate for Rust

Copyright © 2023 Gil Poiares-Oliveira <gil@poiares-oliveira.com>.
All rights reserved.

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
 If a copy of the MPL was not distributed with this file,
 You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#![doc = include_str!("../README.md")]

//pub mod requests;
pub mod client;
pub mod email;

#[allow(dead_code)]
pub mod structures;

pub use client::OmglolClient;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // These tests are just for debugging, no proper test-driven development here
    // don't use this for anything

    fn init_client() -> (OmglolClient, String) {
        use dotenv::dotenv;
        dotenv().ok();
        let address = env::var("OMGLOL_ADDRESS").unwrap_or("foobar".to_string());
        let api_key = env::var("OMGLOL_API_KEY").unwrap_or("".to_string());
        let client = OmglolClient::new(Some(api_key));
        println!("Using account {} for testing", &address);

        return (client, address);
    }

    #[tokio::test]
    async fn main() {
        // If you're learning Rust best practices, maybe you should ignore the next
        // 2 lines. This is a test function so does anyone *actually* care?

        //let info = client.get_profile_themes().await;

        //println!("{:#?}", info);

        //let info = requests::dns::get_dns_records(&address, &api_key).await;
        // let info = requests::themes::get_profile_themes().await;
        // let info = get_statuslog_status(&address, "63a8a6b3cbdc5").await;
        // let info = get_all_statuslog_statuses(&address).await;

        //let my_client = OmglolClient::new(address, api_key=api_key);

        // let destinations = vec![
        //     EmailAddress::from_str("foo@example.net").unwrap(),
        //     EmailAddress::from_str("bar@example.net").unwrap(),
        // ];
        // let info =
        //     requests::email::set_forwarding_addresses(&address, &destinations, &api_key).await;
        //println!("response = {:#?}", info);
    }

    #[tokio::test]
    async fn get_service_status() {
        let (client, _) = init_client();
        let service_status = client.service_status().await;
        println!("{:#?}", service_status);
    }

    #[tokio::test]
    async fn get_profile_themes() {
        let (client, _) = init_client();
        let info = client.get_profile_themes().await;
        println!("{:#?}", info);
    }

    #[tokio::test]
    async fn get_dns_records() {
        let (client, address) = init_client();
        let response = client.get_dns_records(&address).await;
        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn get_web_page() {
        let (client, address) = init_client();
        let response = client.get_web_page(&address).await;
        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn query_unauthorized_endpoint() {
        let (client, _) = init_client();
        let response = client.get_web_page("foobar").await;
        println!("{:#?}", response);
    }

    #[test]
    fn dns_display() {
        assert!(format!("{}", structures::DNStype::A) == "A");
    }
}
