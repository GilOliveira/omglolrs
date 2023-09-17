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
pub struct AccountResponse {
    /// Status message returned by the API
    pub message: String,

    /// Email associated with the account
    pub email: String,

    /// Name associated with the account
    pub name: String, // TODO: Check if this can be null

    /// API key
    pub api_key: String,

    /// Timestamp of account creation
    pub created: TimeStrings,

    /// User-defined account settings
    pub settings: AccountSettings,
}

#[derive(Deserialize, Debug)]
pub struct AccountSettings {
    /// Email address of account owner
    pub owner: String,

    /// Opt-in for email communications
    pub communication: Option<String>, // IMPROVEMENT: Remove "Option" if patched upstream https://github.com/neatnik/omg.lol/issues/613
    /// Opt-in for email communications
    pub date_format: Option<String>, // IMPROVEMENT: Remove "Option" if patched upstream https://github.com/neatnik/omg.lol/issues/613

    /// Which web editor the user chose to use
    pub web_editor: String, // TODO: Check if this can be null, it seems so in the official docs
}

/// Response to a request to determine if an account is verified
#[derive(Deserialize, Debug)]
pub struct Verification {
    /// Status message returned by the API
    pub message: String,

    /// True if the account is verified, false otherwise
    pub verified: bool,
}

/// Address expiration time in different string formats
#[derive(Deserialize, Debug)]
pub struct Expiration {
    /// Status message returned by the API
    pub message: String,

    /// True if the domain has expired, false otherwise
    pub expired: bool,

    /// False if the domain has a lifetime registration, true otherwise
    pub will_expire: Option<bool>,

    /// Unix time
    pub unix_epoch_time: Option<String>,

    /// Time in [ISO 7601](https://www.iso.org/iso-8601-date-and-time-format.html) format
    pub iso_8601_time: Option<String>,

    /// Time in [RFC 2822](https://datatracker.ietf.org/doc/html/rfc2822) format
    pub rfc_2822_time: Option<String>,

    /// Relative time in English
    pub relative_time: Option<String>,
}

/// Address registered with omg.lol
#[derive(Deserialize, Debug)]
pub struct Address {
    /// String of address, without ".omg.lol"
    pub address: String,

    /// Status message returned by the API
    pub message: String,

    /// Time of address registration
    pub registration: TimeStrings,

    /// Time of address expiration
    pub expiration: Expiration,

    /// Verification status of address
    pub verification: Verification,

    /// Email of the address' owner, if it exists
    pub owner: Option<String>,
}

/// Successful DNS records request response
#[derive(Deserialize, Debug)]
pub struct DNSrecords {
    /// Status message returned by the API
    pub message: String,

    /// Vector of DNS records
    pub dns: Vec<DNSrecord>,
}

/// DNS record
#[derive(Deserialize, Debug)]
pub struct DNSrecord {
    /// Record ID
    pub id: i32,

    /// Domain relative to omg.lol zone
    pub name: String,

    /// Entry data/target
    pub data: String,

    /// Record priority, if applicable
    pub priority: Option<i32>,

    /// Record TTL (time to live)
    pub ttl: i32,

    /// Creation timestamp in
    /// [ISO 7601](https://www.iso.org/iso-8601-date-and-time-format.html) format, if applicable
    pub created_at: Option<String>,

    /// Last update timestamp in
    /// [ISO 7601](https://www.iso.org/iso-8601-date-and-time-format.html) format, if applicable
    pub updated_at: Option<String>,

    /// DNS record type
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
    /// Status message returned by the API
    pub message: String,
    pub destination_string: String,
    pub destination_array: Vec<String>,
    pub address: String,
    pub email_address: String,
}

/// Struct for when the server returns a String message
#[derive(Deserialize, Debug)]
pub struct MessageResponse {
    /// Status message returned by the API
    pub message: String,
}

/// Response to a REST API request.
#[derive(Deserialize, Debug)]
pub struct RequestResponse<T> {
    pub request: RequestStatus,
    pub response: T,
}

/// Status of a request
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
    /// Message provided by the API server in response to some requests.
    pub message: Option<String>,
}

/// Response to a query to a Now page endpoint
#[derive(Deserialize, Debug)]
pub struct NowResponse {
    pub message: String,
    pub now: Vec<NowPage>,
}

/// Now page
#[derive(Deserialize, Debug)]
pub struct NowPage {
    pub content: String,
    pub updated: u32,
    pub listed: u8,
    pub nudge: u8,
    pub metadata: String,
}

/// Response to a Now Garden endpoint
#[derive(Deserialize, Debug)]
pub struct NowGardenResponse {
    pub message: String,
    pub garden: Vec<NowGarden>,
}

/// A Now Garden
#[derive(Deserialize, Debug)]
pub struct NowGarden {
    pub address: String,
    pub url: String,
    pub updated: TimeStrings,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

/// Response for a Paste request
#[derive(Deserialize, Debug)]
pub struct PasteResponse {
    /// Status message returned by the API
    pub message: String,
    pub pastebin: Paste,
}

/// Response for a Pastebin request
#[derive(Deserialize, Debug)]
pub struct PastebinResponse {
    /// Status message returned by the API
    pub message: String,
    pub success: Vec<Paste>,
}

/// Pastebin paste
#[derive(Deserialize, Serialize, Debug)]
pub struct Paste {
    pub title: String,
    pub content: String,

    #[serde(skip_serializing)]
    pub modified_on: Option<String>,
}

/// A Paste from [paste.lol](https://paste.lol)
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
    /// Status message returned by the API
    pub message: String,
    pub purl: Purl,
}

#[derive(Deserialize, Debug)]
pub struct PurlsResponse {
    /// Status message returned by the API
    pub message: String,
    pub purls: Vec<Purl>,
}

#[derive(Deserialize, Debug)]
pub struct Purl {
    pub name: String,
    pub url: String,
    pub counter: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct ServiceStatus {
    /// Status message returned by the API
    pub message: String,
    pub members: i32,
    pub addresses: i32,
    pub profiles: i32,
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
    /// Status message returned by the API
    pub message: String,
    pub id: String,
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct StatuslogUpdateResponse {
    /// Status message returned by the API
    pub message: String,
    pub id: String,
    pub url: String,
}

/// Response struct for a successful statuslog query all statuses request
#[derive(Deserialize, Debug)]
pub struct StatuslogAllStatuses {
    /// Status message returned by the API
    pub message: String,
    pub statuses: Vec<Status>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    /// A status log entry.
    pub emoji: String,
    pub content: String,
    pub external_url: Option<String>,

    #[serde(skip_serializing)]
    pub id: String,

    #[serde(skip_serializing)]
    pub address: String,

    #[serde(skip_serializing)]
    pub created: String,

    #[serde(skip_serializing)]
    pub relative_time: String,
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
    pub bio: String,

    #[serde(skip_serializing)]
    pub css: String,

    #[serde(skip_serializing)]
    pub message: String,
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
    pub message: String,
    pub id: String,
    pub status: String,
    pub url: String,
    pub external_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ProfileThemes {
    pub message: String,
    pub themes: HashMap<String, Theme>,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub created: String,
    pub updated: String,
    pub author: String,
    pub author_url: String,
    pub version: String,
    pub license: String,
    pub description: String,
    pub preview_css: String,
    pub sample_profile: String,
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

/// Response to a request for weblog entries
#[derive(Deserialize, Debug)]
pub struct WeblogEntriesResponse {
    /// Status message provided by the API
    pub message: String,

    /// A vector of `WeblogEntry` structs.
    pub entries: Vec<WeblogEntry>,
}

/// API response for a weblog entry GET request.
#[derive(Deserialize, Debug)]
pub struct WeblogEntryResponse {
    /// Status message povided by the API
    pub message: String,

    /// Weblog entry
    #[serde(alias = "post")]
    pub entry: WeblogEntry,
}

/// A weblog entry
#[derive(Deserialize, Debug)]
pub struct WeblogEntry {
    pub location: String,
    pub title: String,
    pub date: u32,
    pub status: String,
    pub body: String,
    pub source: String,
    pub metadata: WeblogMetadata,
    pub output: String,
    pub entry: String,

    #[serde(rename(deserialize = "type"))]
    entry_type: String,
}

/// Metadata for a weblog entry.
#[derive(Deserialize, Debug)]
pub struct WeblogMetadata {
    pub date: String,
    pub slug: String,
}

/// API response for a weblog configuration GET request.
#[derive(Deserialize, Debug)]
pub struct WeblogConfigurationResponse {
    pub message: String,
    pub configuration: WeblogConfigurationFormats,
}

/// Weblog configuration in different formats
#[derive(Deserialize, Debug)]
pub struct WeblogConfigurationFormats {
    pub object: WeblogConfiguration,
    pub json: String,
    pub raw: String,
}

/// A weblog's configuration.
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

/// API response for a weblog template GET request.
#[derive(Deserialize, Debug)]
pub struct WeblogTemplateResponse {
    pub message: String,
    pub template: String,
}

/// An HTTP request error
#[derive(Debug, Clone)]
pub struct RequestError {
    /// HTTP status code as defined in [RFC 9110](https://httpwg.org/specs/rfc9110.html#overview.of.status.codes).
    pub status_code: u16,
}

impl Error for RequestError {}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request error. HTTP status code: {}.", &self.status_code)
    }
}
