pub use crate::do_html;

/// ### do_forloop(vector, befor_items, befor_item, after_item, after_items)
///
/// Html Template Function
///
/// The `do_forloop` function takes a vector of items and formats them according to specified prefixes
/// and suffixes for the entire collection, as well as for each individual item. The output
/// is a concatenated string representation of the items, encapsulated within the provided
/// separator strings.
///
/// ### Parameters
/// - `vector`: A slice of items to format, where each item must implement the `Display` trait.
/// - `befor_items`: A string that will be prefixed before the whole collection of items.
/// - `befor_item`: A string that will be prefixed before each individual item.
/// - `after_item`: A string that will be suffixed after each individual item.
/// - `after_items`: A string that will be suffixed after the whole collection of items.
///
/// ### Examples
/// ```rust
/// use cans::html::do_forloop;
///
/// let items = vec!["Apples", "Bananas", "Cherries"];
/// let result = do_forloop(&items, "<ul>", "<li>", "</li>", "</ul>");
/// assert_eq!(result, "<ul><li>Apples</li><li>Bananas</li><li>Cherries</li></ul>");
///
/// let float_vector = vec![1.0, 2.0, 3.0];
/// let forloop_float = do_forloop(&float_vector, "", "", "", "");
/// assert_eq!(forloop_float, "123");
/// ```
/// <small>End Fun Doc</small>
pub fn do_forloop<T: std::fmt::Display>(
    vector: &[T],
    befor_items: &str,
    befor_item: &str,
    after_item: &str,
    after_items: &str,
) -> String {
    let items = vector
        .iter()
        .map(|item| format!("{befor_item}{item}{after_item}"))
        .collect::<Vec<_>>()
        .join("");
    format!("{}{}{}", befor_items, items, after_items)
}

/// ### do_text(t)
///
/// Text Conversion Function
///
/// The `do_text` function takes a string slice representing a text and converts it
/// into an owned `String`. This function is typically used for text elements,
/// including titles and other content that are incorporated into HTML templates,
/// ensuring proper ownership and allocation of string data.
///
/// ### Parameters
/// - `t`: A string slice (`&str`) representing the text to be used in
///   HTML templates.
///
/// ### Examples
/// ```rust
/// use cans::html::do_text;
///
/// let title = "Home";
/// let result = do_text(title);
/// assert_eq!(result, "Home");
///
/// let color = do_text("#000");
/// assert_eq!(color, "#000");
/// ```
///
/// ### Usage Context
/// This function is particularly useful in the context of generating dynamic HTML
/// pages where text elements, such as titles, may need to be included as part
/// of an HTML document or template. For instance, it is used in the `do_home_page`
/// function to set the title in the `HOME_TEMPLATE` HTML document:
/// ```rust
/// use cans::html::{do_html, do_text};
///
/// pub const HEAD: &str = r#"<head>
/// <meta charset="UTF-8">
///     <title>{{page_title}} Page</title>
/// </head>"#;
///
/// pub const HOME_TEMPLATE: &str = r#"<!DOCTYPE html>
/// <html>
///   {{HEAD}}
///   <body>
///      Home Page
///   </body>
/// </html>"#;
///
/// pub fn do_home_page() -> String {
///     do_html!(HOME_TEMPLATE, HEAD = HEAD, page_title = do_text("Home"))
/// }
///
/// pub const ABOUT_TEMPLATE: &str = r#"<!DOCTYPE html>
/// <html>
///   {{HEAD}}
///   <body>
///      About Page
///   </body>
/// </html>"#;
///
/// pub fn do_about_page() -> String {
///     do_html!(ABOUT_TEMPLATE, HEAD = HEAD, page_title = do_text("About"))
/// }
/// ```
///
/// This ensures that the text is properly formatted and owned, allowing for
/// efficient rendering and manipulation in templates.
///
/// <small>End Fun Doc</small>
pub fn do_text(t: &str) -> String {
    t.to_string()
}
