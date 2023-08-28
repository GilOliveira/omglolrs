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
