// SPDX-LICENSE-IDENTIFIER: MPL-2.0

/*
Strucutures - omglol crate for Rust

Copyright © 2023 Gil Poiares-Oliveira <gil@poiares-oliveira.com>.
All rights reserved.

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
 If a copy of the MPL was not distributed with this file,
 You can obtain one at https://mozilla.org/MPL/2.0/.
*/

//! Structures corresponding to various API elements

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt, fmt::Display};

// fn return_true() -> bool {
//     // This is the world's more useful function /s
//     // #[serde(default)] needs a function to be supplied.
//     true

#[derive(Deserialize, Debug)]
pub struct AcccountResponse {
    message: String,
    email: String,
    name: String,
    api_key: String,
    created: TimeStrings,
    settings: AccountSettings,
}

#[derive(Deserialize, Debug)]
pub struct AccountSettings {
    owner: String,
    communication: Option<String>, // IMPROVEMENT: Remove "Option" if patched upstream
    date_format: Option<String>,   // https://github.com/neatnik/omg.lol/issues/613
    web_editor: String,
}

#[derive(Deserialize, Debug)]
pub struct Verification {
    message: String,
    pub verified: bool,
}

/// Address expiration time in different string formats
#[derive(Deserialize, Debug)]
pub struct Expiration {
    message: String,
    pub expired: bool,
    pub will_expire: Option<bool>,
    pub unix_epoch_time: Option<String>,
    pub iso_8601_time: Option<String>,
    pub rfc_2822_time: Option<String>,
    pub relative_time: Option<String>,
}

/// omg.lol address
#[derive(Deserialize, Debug)]
pub struct Address {
    address: String,
    message: String,
    registration: TimeStrings,
    expiration: Expiration,
    verification: Verification,
    owner: Option<String>,
}

/// Successful DNS records request response
#[derive(Deserialize, Debug)]
pub struct DNSrecords {
    message: String,
    dns: Vec<DNSrecord>,
}

/// DNS record
#[derive(Deserialize, Debug)]
pub struct DNSrecord {
    pub id: i32,
    pub name: String,
    pub data: String,
    pub priority: Option<i32>,
    pub ttl: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    #[serde(rename = "type")]
    pub record_type: DNStype,
}

/// DNS record types supported by omg.lol DNS routing.
#[derive(Deserialize, Debug)]
pub enum DNStype {
    A,
    AAA,
    CAA,
    CNAME,
    MX,
    NS,
    SRV,
    TXT,
}

impl Display for DNStype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

/// Forwarding addresses for your @omg.lol email address
#[derive(Deserialize, Debug)]
pub struct ForwardingAddresses {
    message: String,
    destination_string: String,
    destination_array: Vec<String>,
    address: String,
    email_address: String,
}

#[derive(Deserialize, Debug)]
pub struct MessageResponse {
    message: String,
}

/// Response to a REST API request.
#[derive(Deserialize, Debug)]
pub struct RequestResponse<T> {
    pub request: RequestStatus,
    pub response: T,
}

#[derive(Deserialize, Debug)]
pub struct RequestStatus {
    /// HTTP status code of the request response
    pub status_code: u16,
    /// `true` if request completed successfully, `false` otherwise.
    pub success: bool,
}

/// String representations of a moment in time according to different protocols.
#[derive(Deserialize, Debug)]
pub struct TimeStrings {
    /// String of seconds elapsed since the UNIX epoch (1970-01-01 00:00:00 UTC)
    pub unix_epoch_time: String,
    /// Time in [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html) format.
    pub iso_8601_time: String,
    /// Time in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    pub rfc_2822_time: String,
    /// Relative time string, e.g. "20 minutes ago".
    pub relative_time: String,
    /// Message provided by the API server in respnse to some requests.
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NowResponse {
    message: String,
    now: Vec<NowPage>,
}

#[derive(Deserialize, Debug)]
pub struct NowPage {
    content: String,
    updated: u32,
    listed: u8,
    nudge: u8,
    metadata: String,
}

#[derive(Deserialize, Debug)]
pub struct NowGardenResponse {
    message: String,
    garden: Vec<NowGarden>,
}

#[derive(Deserialize, Debug)]
pub struct NowGarden {
    address: String,
    url: String,
    updated: TimeStrings,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Deserialize, Debug)]
pub struct PasteResponse {
    message: String,
    pastebin: Paste,
}

#[derive(Deserialize, Debug)]
pub struct PastebinResponse {
    message: String,
    success: Vec<Paste>,
}

/// Pastebin paste
#[derive(Deserialize, Serialize, Debug)]
pub struct Paste {
    pub title: String,
    pub content: String,

    #[serde(skip_serializing)]
    modified_on: Option<String>,
}

impl Paste {
    /// New instance of `Paste`
    pub fn new(title: String, content: String) -> Paste {
        Paste {
            title,
            content,
            modified_on: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PurlResponse {
    message: String,
    purl: Purl,
}

#[derive(Deserialize, Debug)]
pub struct PurlsResponse {
    message: String,
    purls: Vec<Purl>,
}

#[derive(Deserialize, Debug)]
pub struct Purl {
    name: String,
    url: String,
    counter: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct ServiceStatus {
    message: String,
    members: i32,
    addresses: i32,
    profiles: i32,
}

pub trait ContentAsJSON {
    fn json_content(&self) -> String {
        panic!("Error converting to JSON content.")
    }
}

impl ContentAsJSON for String {
    fn json_content(&self) -> String {
        format!("{{\"content\": \"{}\"}}", &self)
    }
}

#[derive(Deserialize, Debug)]
pub struct StatuslogResponseArray {
    message: String,
    id: String,
    status: Status,
}

#[derive(Deserialize, Debug)]
pub struct StatuslogUpdateResponse {
    message: String,
    id: String,
    url: String,
}

/// Response struct for a successful statuslog query all statuses request
#[derive(Deserialize, Debug)]
pub struct StatuslogAllStatuses {
    message: String,
    statuses: Vec<Status>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    /// A status log entry.
    pub emoji: String,
    pub content: String,
    pub external_url: Option<String>,

    #[serde(skip_serializing)]
    id: String,

    #[serde(skip_serializing)]
    pub address: String,

    #[serde(skip_serializing)]
    created: String,

    #[serde(skip_serializing)]
    relative_time: String,
}

impl Status {
    /// Create a new instance of `Status`
    ///
    /// Arguments:
    /// * `emoji`, your status' emoji
    /// * `content`, The text content of your status
    /// * `external_url`, An optional external URL for your status
    pub fn new(emoji: String, content: String, external_url: Option<String>) -> Status {
        Status {
            emoji,
            content,
            external_url,
            id: "".to_string(),
            address: "".to_string(),
            created: "".to_string(),
            relative_time: "".to_string(),
        }
    }
}

/// Your statuslog's bio.
#[derive(Deserialize, Serialize, Debug)]
pub struct StatuslogBio {
    bio: String,

    #[serde(skip_serializing)]
    css: String,

    #[serde(skip_serializing)]
    message: String,
}

impl StatuslogBio {
    /// Create a new `StatuslogBio` struct.
    ///
    /// Arguments:
    /// * `bio: String` the text content of your bio
    pub fn new(bio: String) -> StatuslogBio {
        StatuslogBio {
            bio,
            css: "".to_string(),
            message: "".to_string(),
        }
    }
}

impl ContentAsJSON for StatuslogBio {
    fn json_content(&self) -> String {
        format!("{{\"content\": \"{}\"}}", &self.message)
    }
}

#[derive(Deserialize, Debug)]
pub struct StatusPostResponse {
    message: String,
    id: String,
    status: String,
    url: String,
    external_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ProfileThemes {
    message: String,
    themes: HashMap<String, Theme>,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    id: String,
    name: String,
    created: String,
    updated: String,
    author: String,
    author_url: String,
    version: String,
    license: String,
    description: String,
    preview_css: String,
    sample_profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Web {
    pub content: String,

    /// Status message provided by the API
    #[serde(skip_serializing)]
    pub message: String,

    /// CSS profile code
    #[serde(skip_serializing)]
    pub css: String,

    /// Additional `<head>` elements
    #[serde(skip_serializing)]
    pub head: String,

    /// Weather the profile has been [verified](https://home.omg.lol/info/profile-verification).
    #[serde(skip_serializing)]
    pub verified: Option<i8>,

    /// Profile picture, `Some` with a URL `String` or `None` if not defined.
    #[serde(skip_serializing)]
    pub pfp: Option<String>,

    #[serde(skip_serializing)]
    pub metadata: String,

    #[serde(skip_serializing)]
    pub branding: String,

    #[serde(rename(deserialize = "type"), skip_serializing)]
    pub page_type: String,
    // Publish currently does not do anything
    //#[serde(skip_deserializing, default = "return_true")]
    //pub publish: bool,
}

impl Web {
    /// Create a new `Web` struct.
    ///
    /// Arguments:
    /// * `content: String` with the web page content.
    pub fn new(content: String) -> Web {
        Web {
            //publish,
            content,
            message: " ".to_string(),
            css: "".to_string(),
            head: "".to_string(),
            verified: None,
            pfp: None,
            metadata: "".to_string(),
            branding: "".to_string(),
            page_type: "".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct WeblogEntriesResponse {
    message: String,
    entries: Vec<WeblogEntry>,
}

#[derive(Deserialize, Debug)]
pub struct WeblogEntryResponse {
    message: String,

    #[serde(alias = "post")]
    entry: WeblogEntry,
}

#[derive(Deserialize, Debug)]
pub struct WeblogEntry {
    location: String,
    title: String,
    date: u32,
    status: String,
    body: String,
    source: String,
    metadata: WeblogMetadata,
    output: String,
    entry: String,

    #[serde(rename(deserialize = "type"))]
    entry_type: String,
}

#[derive(Deserialize, Debug)]
pub struct WeblogMetadata {
    date: String,
    slug: String,
}

#[derive(Deserialize, Debug)]
pub struct WeblogConfigurationResponse {
    message: String,
    configuration: WeblogConfigurationFormats,
}

#[derive(Deserialize, Debug)]
pub struct WeblogConfigurationFormats {
    object: WeblogConfiguration,
    json: String,
    raw: String,
}

#[derive(Deserialize, Debug)]
pub struct WeblogConfiguration {
    pub weblog_title: String,
    pub weblog_description: String,
    pub author: String,
    pub separator: String,
    pub tag_path: String,
    pub timezone: String,
    pub date_format: String,
    pub default_post: String,
    pub feed_post_count: String,
    pub recents_posts_format: String,
    pub post_list_format: String,
    pub search_satus: String,
    pub search_status_success_message: String,
    pub search_results_failure_message: String,
    pub search_results_format: String,
}

#[derive(Deserialize, Debug)]
pub struct WeblogTemplateResponse {
    pub message: String,
    pub template: String,
}

#[derive(Debug, Clone)]
pub struct RequestError {
    pub status_code: u16,
}

impl Error for RequestError {}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request error. HTTP status code: {}.", &self.status_code)
    }
}
