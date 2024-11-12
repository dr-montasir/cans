/// ### do_html!($html, $key, and $val)
///
/// Macro Rules
///
/// The `do_html` macro takes an HTML string along with key-value pairs and replaces placeholders in the HTML (formatted as `{{key}}`) with the corresponding values.
/// It returns the processed HTML string with the substitutions applied.
///
/// ### Parameters
/// - `$html`: The HTML string containing placeholders for substitution (e.g., `"<p>Hello, {{name}}!</p>"`).
/// - `$key`: The identifier for each placeholder in the HTML (e.g., `name`).
/// - `$val`: The value that replaces the corresponding placeholder in the HTML (e.g., `"Alice"`).
///
/// ### Examples
/// ```rust
/// // use cans::html::do_html;
/// use cans::do_html;
///
/// let template = "<p>Hello, {{name}}! Welcome to {{place}}.</p>";
/// let result = do_html!(template, name = "Dear", place = "CANS Template");
/// assert_eq!(result, "<p>Hello, Dear! Welcome to CANS Template.</p>");
/// ```
/// <small>End Mac Doc</small>
#[macro_export]
macro_rules! do_html {
    // Accept an HTML block and key-value pairs for substitution
    ($html:expr, $($key:ident = $val:expr),*) => {{
        let mut html_string = $html.to_string();

        // Iterate over all key-value pairs and replace placeholders
        $(
            // Replace placeholders like `{{name}}` in the HTML string
            let val = format!("{}", $val);
            html_string = html_string.replace(&format!("{{{{{}}}}}", stringify!($key)), &val);
        )*

        // Return the processed HTML string
        html_string
    }};
}
