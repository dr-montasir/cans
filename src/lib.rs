#![doc(html_logo_url = "https://github.com/dr-montasir/cans/raw/HEAD/cans-logo.svg")]
#![doc = r"<div align='center'><a href='https://github.com/dr-montasir/cans' target='_blank'><img src='https://github.com/dr-montasir/cans/raw/HEAD/cans-logo.svg' alt='CANS' width='80' height='auto' /></a><br><a href='https://github.com/dr-montasir/cans' target='_blank'>CANS</a><br><br><b>An elegant and lightweight Rust-based literal template engine for managing web content, enhanced with a world module for streamlined regional and city information, as well as robust MIME type management.</b></div>"]

/// An elegant and lightweight Rust-based literal template engine for managing web content,
/// enhanced with a world module for streamlined regional and city information,
/// as well as robust MIME type management.
///
/// ### HTML Module
///
/// A set of functions and macros for generating and manipulating various content.
pub mod content;

/// ### Mime Module
///
/// A set of functions for managing MIME types using a HashMap.
pub mod mime;

/// ### Rules Module
///
/// A module that encapsulates the rules and functionalities of the `do_html` macro.
pub mod rules;

// ### World Module
///
/// A module that encapsulates managing country details, including insertion, deletion, retrieval, and sorting.
/// It enables dynamic management of a collection of countries and their associated cities.
/// Users can efficiently reset the dataset to accommodate new data for testing purposes.
/// The module promotes data integrity and optimal memory management through precise control over entries.
/// It allows for easy querying and manipulation of country and city information to support various applications.
pub mod world;
