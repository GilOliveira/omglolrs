// SPDX-LICENSE-IDENTIFIER: MPL-2.0

/*
Email - omglol crate for Rust

Copyright © 2023 Gil Poiares-Oliveira <gil@poiares-oliveira.com>.
All rights reserved.

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
 If a copy of the MPL was not distributed with this file,
 You can obtain one at https://mozilla.org/MPL/2.0/.
*/

//! Helper to format email addresses in the format that the API expects

use email_address::EmailAddress;

/// Format a `Vec<EmailAddress>` to a String in the JSON format the API expects
///
/// # Arguments
/// * `addresses` - A `Vec<EmailAddress>` containing the forwarding email address(es)
/// # Output
/// `String` in the JSON format the API endpoint expects
pub fn format_addresses_string(addresses: &Vec<EmailAddress>) -> String {
    // This code is will probably make any serious Rust developer hide in shame
    // but the endpoint expects a weird format and... well, if it works...
    let mut body_string = String::from(format!("{{\"destination\": \""));
    body_string.push_str(&addresses[0].to_string().as_str());

    if addresses.len() > 1 {
        for address in addresses {
            body_string.push_str(", ");
            body_string.push_str(&address.as_str());
        }
    }

    body_string.push_str("\"}");

    return body_string;
}
