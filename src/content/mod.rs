pub use crate::do_replace;
pub use crate::do_html;
pub use crate::do_xml;
pub use crate::do_json;

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
/// use cans::content::do_forloop;
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
/// use cans::content::do_text;
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
/// use cans::content::{do_html, do_text};
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

/// ### alpine(version)
///
/// Alpine.js Script Tag Generator
///
/// The `alpine` function generates an HTML `<script>` tag string that loads the specified version
/// of the Alpine.js library from a CDN. You provide the version as a string slice, and the function
/// returns a formatted string containing the script tag with the correct version embedded.
///
/// ### Parameters
/// - `version`: A string slice (`&str`) representing the version of Alpine.js to include,
///   for example `"3.15.0"` or `"latest"` for the most recent version.
///
/// ### Examples
/// ```rust
/// use cans::content::alpine;
///
/// let script_tag = alpine("3.15.0");
/// assert_eq!(script_tag, r#"<script defer src="https://unpkg.com/alpinejs@3.15.0/dist/cdn.min.js"></script>"#);
///
/// let latest_script_tag = alpine("latest");
/// assert_eq!(latest_script_tag, r#"<script defer src="https://unpkg.com/alpinejs@latest/dist/cdn.min.js"></script>"#);
/// ```
///
/// ### Usage Context
/// This function is useful when dynamically generating HTML pages or templates that need to include
/// the Alpine.js library. By passing in the desired version, you can easily control which version
/// of Alpine is loaded, facilitating version management and updates in your web projects.
///
/// For example, in a server-side rendered HTML template:
/// ```rust
/// use cans::content::alpine;
/// let head_content = format!("<head>{}", alpine("3.15.0"));
/// ```
///
/// This ensures the correct script tag is embedded in the HTML, enabling Alpine.js functionalities.
///
/// <small>End Fun Doc</small>
pub fn alpine(version: &str) -> String {
    format!(r#"<script defer src="https://unpkg.com/alpinejs@{}/dist/cdn.min.js"></script>"#, version)
}

/// ### chart_js (version)
///
/// Chart.js Script Tag Generator
///
/// The `chart_js` function generates an HTML `<script>` tag string that loads the specified version
/// of the Chart.js library from a CDN. You provide the version as a string slice, and the function
/// returns a formatted string containing the script tag with the correct version embedded.
///
/// ### Parameters
/// - `version`: A string slice (`&str`) representing the version of Chart.js to include,
///   for example `"4.2.1"` or `"latest"` for the most recent version.
///
/// ### Examples
/// ```rust
/// use cans::content::chart_js;
///
/// let script_tag = chart_js("4.2.1");
/// assert_eq!(script_tag, r#"<script src="https://cdn.jsdelivr.net/npm/chart.js@4.2.1/dist/chart.umd.min.js"></script>"#);
///
/// let latest_script_tag = chart_js("latest");
/// assert_eq!(latest_script_tag, r#"<script src="https://cdn.jsdelivr.net/npm/chart.js@latest/dist/chart.umd.min.js"></script>"#);
/// ```
///
/// ### Usage Context
/// This function is useful when dynamically generating HTML pages or templates that need to include
/// the Chart.js library. By passing in the desired version, you can easily control which version
/// of Chart.js is loaded, facilitating version management and updates in your web projects.
///
/// For example, in a server-side rendered HTML template:
/// ```rust
/// use cans::content::chart_js;
/// let head_content = format!("<head>{}</head>", chart_js("4.2.1"));
/// ```
///
/// This ensures the correct script tag is embedded in the HTML, enabling Chart.js functionalities.
///
/// <small>End Fun Doc</small>
pub fn chart_js(version: &str) -> String {
    format!(r#"<script src="https://cdn.jsdelivr.net/npm/chart.js@{}/dist/chart.umd.min.js"></script>"#, version)
}