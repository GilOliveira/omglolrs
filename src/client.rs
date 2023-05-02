// SPDX-LICENSE-IDENTIFIER: MPL-2.0

/*
Client - omglol crate for Rust

Copyright © 2023 Gil Poiares-Oliveira <gil@poiares-oliveira.com>.
All rights reserved.

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
 If a copy of the MPL was not distributed with this file,
 You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::marker::PhantomData;

use crate::email::format_addresses_string;
use crate::structures::*;
use email_address::EmailAddress;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde_json;

macro_rules! api_endpoint (
    ($path: expr) => (
        format!("{}{}", "https://api.omg.lol/", $path)
    );
);

pub(crate) use api_endpoint;

pub struct Auth;
pub struct NoAuth;


/// Client for api.omg.lol
#[derive(Clone)]
pub struct OmglolClient<State = NoAuth> {
    client: Client,
    api_key: Option<String>,
    state: PhantomData<State>,
}

impl OmglolClient<Auth> {
    pub async fn get_dns_records(
        &self,
        address: &str,
    ) -> Result<RequestResponse<DNSrecords>, Box<dyn std::error::Error>> {
        self.send_request::<DNSrecords>(
            true,
            Method::GET,
            format!("address/{}/dns", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn delete_dns_record(
        &self,
        address: &str,
        id: &str,
    ) -> Result<RequestResponse<DNSrecords>, Box<dyn std::error::Error>> {
        self.send_request::<DNSrecords>(
            true,
            Method::DELETE,
            format!("address/{address}/dns/{id}").as_ref(),
            None,
        )
        .await
    }

    pub async fn get_status(
        &self,
        address: &str,
        id: &str,
    ) -> Result<RequestResponse<StatuslogResponseArray>, Box<dyn std::error::Error>> {
        self.send_request::<StatuslogResponseArray>(
            true,
            Method::GET,
            format!("address/{}/statuses/{}", &address, &id).as_ref(),
            None,
        )
        .await
    }

    pub async fn post_status(
        &self,
        status: &Status,
    ) -> Result<RequestResponse<StatuslogResponseArray>, Box<dyn std::error::Error>> {
        self.send_request::<StatuslogResponseArray>(
            true,
            Method::POST,
            format!("address/{}/statuses", &status.address).as_ref(),
            Some(serde_json::to_string(&status)?),
        )
        .await
    }

    pub async fn update_status(
        &self,
        status: &Status,
    ) -> Result<RequestResponse<StatuslogUpdateResponse>, Box<dyn std::error::Error>> {
        self.send_request::<StatuslogUpdateResponse>(
            true,
            Method::POST,
            format!("address/{}/status", &status.address).as_ref(),
            Some(serde_json::to_string(&status)?),
        )
        .await
    }

    pub async fn update_statuslog_bio<T: ContentAsJSON>(
        &self,
        bio: T,
        address: &str,
    ) -> Result<RequestResponse<StatuslogBio>, Box<dyn std::error::Error>> {
        self.send_request::<StatuslogBio>(
            true,
            Method::POST,
            format!("address/{}/statuses/bio", &address).as_ref(),
            Some(bio.json_content()),
        )
        .await
    }

    // Email

    pub async fn get_forwarding_addresses(
        &self,
        address: &str,
    ) -> Result<RequestResponse<ForwardingAddresses>, Box<dyn std::error::Error>> {
        self.send_request::<ForwardingAddresses>(
            true,
            Method::GET,
            format!("address/{}/email", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn set_forwarding_addresses(
        &self,
        address: &str,
        destination: &Vec<EmailAddress>,
    ) -> Result<RequestResponse<ForwardingAddresses>, Box<dyn std::error::Error>> {
        self.send_request::<ForwardingAddresses>(
            true,
            Method::GET,
            format!("address/{}/email", &address).as_ref(),
            Some(format_addresses_string(destination)),
        )
        .await
    }

    // Pastebin

    pub async fn get_pastebin(
        &self,
        address: &str,
    ) -> Result<RequestResponse<PastebinResponse>, Box<dyn std::error::Error>> {
        self.send_request::<PastebinResponse>(
            true,
            Method::GET,
            format!("address/{}/pastebin", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn create_weblog_entry(
        &self,
        content: &str,
        entry_id: &str,
        address: &str,
    ) -> Result<RequestResponse<WeblogEntryResponse>, Box<dyn std::error::Error>> {
        self.send_request::<WeblogEntryResponse>(
            true,
            Method::POST,
            format!("address/{}/weblog/entry/{}", address, entry_id).as_ref(),
            Some(content.to_string()),
        )
        .await
    }

    pub async fn update_weblog_configuration(
        &self,
        configuration: &str,
        address: &str,
    ) -> Result<RequestResponse<WeblogEntryResponse>, Box<dyn std::error::Error>> {
        self.send_request::<WeblogEntryResponse>(
            true,
            Method::POST,
            format!("address/{}/weblog/template", address).as_ref(),
            Some(configuration.to_string()),
        )
        .await
    }
    pub async fn get_purl(
        &self,
        address: &str,
        purl_address: &str,
    ) -> Result<RequestResponse<PurlResponse>, Box<dyn std::error::Error>> {
        self.send_request::<PurlResponse>(
            true,
            Method::GET,
            format!("address/{}/purl/{purl_address}", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_all_purls(
        &self,
        address: &str,
    ) -> Result<RequestResponse<PurlsResponse>, Box<dyn std::error::Error>> {
        self.send_request::<PurlsResponse>(
            true,
            Method::GET,
            format!("address/{}/purls", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn delete_purl(
        &self,
        address: &str,
        purl_address: &str,
    ) -> Result<RequestResponse<MessageResponse>, Box<dyn std::error::Error>> {
        self.send_request::<MessageResponse>(
            true,
            Method::DELETE,
            format!("address/{}/purl/{purl_address}", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_account_info(
        &self,
        email: &EmailAddress,
    ) -> Result<RequestResponse<AcccountResponse>, Box<dyn std::error::Error>> {
        self.send_request::<AcccountResponse>(
            true,
            Method::GET,
            format!("account/{}/info", email).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_private_address_info(
        &self,
        address: &str,
    ) -> Result<RequestResponse<Address>, Box<dyn std::error::Error>> {
        self.send_request::<Address>(
            true,
            Method::GET,
            format!("account/{}/info", address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_address_expiration(
        &self,
        address: &str,
    ) -> Result<RequestResponse<Expiration>, Box<dyn std::error::Error>> {
        self.send_request::<Expiration>(
            true,
            Method::GET,
            format!("account/{}/expiration", address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_web_page(
        &self,
        address: &str,
    ) -> Result<RequestResponse<Web>, Box<dyn std::error::Error>> {
        self.send_request::<Web>(
            true,
            Method::GET,
            format!("address/{}/web", address).as_ref(),
            None,
        )
        .await
    }

    pub async fn update_web_page(
        &self,
        web: &Web,
        address: &str,
    ) -> Result<RequestResponse<MessageResponse>, Box<dyn std::error::Error>> {
        self.send_request::<MessageResponse>(
            true,
            Method::POST,
            format!("address/{}/web", address).as_ref(),
            Some(serde_json::to_string(&web)?),
        )
        .await
    }

    pub async fn get_weblog_entry(
        &self,
        entry_id: &str,
        address: &str,
    ) -> Result<RequestResponse<WeblogEntryResponse>, Box<dyn std::error::Error>> {
        self.send_request::<WeblogEntryResponse>(
            true,
            Method::GET,
            format!("address/{}/weblog/entry/{}", address, entry_id).as_ref(),
            None,
        )
        .await
    }

    pub async fn delete_weblog_entry(
        &self,
        entry_id: &str,
        address: &str,
    ) -> Result<RequestResponse<MessageResponse>, Box<dyn std::error::Error>> {
        self.send_request::<MessageResponse>(
            true,
            Method::DELETE,
            format!("address/{}/weblog/delete/{}", address, entry_id).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_weblog_configuration(
        &self,
        address: &str,
    ) -> Result<RequestResponse<WeblogConfigurationResponse>, Box<dyn std::error::Error>> {
        self.send_request::<WeblogConfigurationResponse>(
            true,
            Method::GET,
            format!("address/{}/weblog/configuration", address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_weblog_template(
        &self,
        address: &str,
    ) -> Result<RequestResponse<WeblogTemplateResponse>, Box<dyn std::error::Error>> {
        self.send_request::<WeblogTemplateResponse>(
            true,
            Method::GET,
            format!("address/{}/weblog/template", address).as_ref(),
            None,
        )
        .await
    }

    pub async fn update_weblog_template(
        &self,
        template: &str,
        address: &str,
    ) -> Result<RequestResponse<WeblogTemplateResponse>, Box<dyn std::error::Error>> {
        self.send_request::<WeblogTemplateResponse>(
            true,
            Method::POST,
            format!("address/{}/weblog/template", address).as_ref(),
            Some(template.to_string()),
        )
        .await
    }

    pub async fn delete_paste(
        &self,
        address: &str,
        title: &str,
    ) -> Result<RequestResponse<MessageResponse>, Box<dyn std::error::Error>> {
        self.send_request::<MessageResponse>(
            false,
            Method::DELETE,
            format!("address/{}/pastebin/{title}", &address).as_ref(),
            None,
        )
        .await
    }
}

impl OmglolClient<NoAuth> {

    /// Create an authenticated `OmglolClient`.
    ///
    /// This client is able to access private endpoints.
    /// 
    /// Example:
    /// ```rust
    /// let client = OmglolClient::new()
    /// let client = client.auth("YOUR_API_KEY".to_string())
    /// ```
    pub fn auth(&self, api_key: String) -> OmglolClient<Auth> {
        OmglolClient {
            client: self.client.to_owned(),
            api_key: Some(api_key),
            state: PhantomData,
        }
    }
}

impl OmglolClient {
    /// Create a new `OmglolClient`.
    ///
    /// The client is created in unauthenticated form, i.e. restricted to
    /// methods that rely on public endpoints only.
    ///
    /// Usage:
    /// ```rust
    /// let client = OmglolClient::new()
    /// ```
    pub fn new() -> OmglolClient<NoAuth> {
        OmglolClient {
            client: Client::new(),
            api_key: None,
            state: PhantomData,
        }
    }

    pub async fn get_profile_themes(
        &self,
    ) -> Result<RequestResponse<ProfileThemes>, Box<dyn std::error::Error>> {
        self.send_request::<ProfileThemes>(false, Method::GET, "theme/list", None)
            .await
    }

    pub async fn service_status(
        &self,
    ) -> Result<RequestResponse<ServiceStatus>, Box<dyn std::error::Error>> {
        self.send_request::<ServiceStatus>(false, Method::GET, "service/info", None)
            .await
    }

    pub async fn get_statuslog_bio(
        &self,
        address: &str,
    ) -> Result<RequestResponse<StatuslogBio>, Box<dyn std::error::Error>> {
        self.send_request::<StatuslogBio>(
            false,
            Method::GET,
            format!("address/{}/statuses/bio", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_listed_pastes(
        &self,
        address: &str,
    ) -> Result<RequestResponse<PastebinResponse>, Box<dyn std::error::Error>> {
        self.send_request::<PastebinResponse>(
            false,
            Method::GET,
            format!("address/{}/pastebin", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_paste(
        &self,
        address: &str,
        title: &str,
    ) -> Result<RequestResponse<PasteResponse>, Box<dyn std::error::Error>> {
        self.send_request::<PasteResponse>(
            false,
            Method::GET,
            format!("address/{}/pastebin/{title}", &address).as_ref(),
            None,
        )
        .await
    }

    pub async fn upload_paste(
        &self,
        address: &str,
        paste: Paste,
    ) -> Result<RequestResponse<PasteResponse>, Box<dyn std::error::Error>> {
        self.send_request::<PasteResponse>(
            false,
            Method::POST,
            format!("address/{}/pastebin", &address).as_ref(),
            Some(serde_json::to_string(&paste)?),
        )
        .await
    }

    pub async fn get_public_address_info(
        &self,
        address: &str,
    ) -> Result<RequestResponse<Address>, Box<dyn std::error::Error>> {
        self.send_request::<Address>(
            false,
            Method::GET,
            format!("account/{}/info", address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_latest_weblog_post(
        &self,
        address: &str,
    ) -> Result<RequestResponse<WeblogEntryResponse>, Box<dyn std::error::Error>> {
        self.send_request::<WeblogEntryResponse>(
            false,
            Method::GET,
            format!("address/{}/weblog/post/latest", address).as_ref(),
            None,
        )
        .await
    }

    pub async fn get_all_statuses(
        &self,
        address: &str,
    ) -> Result<RequestResponse<StatuslogAllStatuses>, Box<dyn std::error::Error>> {
        self.send_request::<StatuslogAllStatuses>(
            false,
            Method::GET,
            format!("address/{}/statuses", &address).as_ref(),
            None,
        )
        .await
    }
}


/// OmglolClient allows you to make authenticated or unauthenticated REST API
/// requests.
impl<State> OmglolClient<State> {
    async fn send_request<T>(
        &self,
        authenticate: bool,
        method: Method,
        uri: &str,
        body: Option<String>,
    ) -> Result<RequestResponse<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let reqwest_client = &self.client;
        let mut req = reqwest_client.request(method, api_endpoint!(uri));

        if authenticate {
            req = req.bearer_auth(&self.api_key.as_ref().unwrap().to_string());
        }

        if body.is_some() {
            req = req.body(body.unwrap());
        }

        #[cfg(debug_assertions)]
        dbg!(&req);

        let resp = req.send().await?;

        let raw_res = match &resp.status().as_u16() {
            _status_code @ 200 => resp.text().await?,
            status_code => {
                return Err(Box::new(RequestError {
                    status_code: *status_code,
                }))
            }
        };

        #[cfg(debug_assertions)]
        dbg!(&raw_res);

        let res: RequestResponse<T> = serde_json::from_str(&raw_res).unwrap();

        Ok(res)
    }
}
