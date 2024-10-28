#![doc(
    html_logo_url = "https://github.com/dr-montasir/cans/raw/HEAD/cans-logo.svg?sanitize=true",
    html_root_url = "https://docs.rs/cans/latest/cans"
)]

/// ### HTML Module
///
/// A set of functions and macros for generating and manipulating HTML content.
pub mod html;

/// ### JSON Module
///
/// A module for parsing and managing JSON-like key-value pairs.
pub mod json;

/// ### Mime Module
///
/// A set of functions for managing MIME types using a HashMap.
pub mod mime;

/// ### Rules Module
///
/// A module that encapsulates the rules and functionalities of the `do_html` and `do_json` macros.
pub mod rules;
