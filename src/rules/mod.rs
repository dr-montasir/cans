/// ### do_replace!($content, $key, and $val)
///
/// Macro Rules
///
/// The `do_replace` macro takes any content string or data (e.g., code, file content, templates)
/// along with key-value pairs and replaces placeholders in the content (formatted as `{{key}}`) with the corresponding values.
/// This macro is highly versatile and can manipulate any content of any programming language or file by substituting placeholders.
///
/// ### Parameters
/// - `$content`: The content string containing placeholders for substitution (e.g., `"<p>Hello, {{name}}!</p>"`).
/// - `$key`: The identifier for each placeholder in the content (e.g., `name`).
/// - `$val`: The value that replaces the corresponding placeholder in the content (e.g., `"Alice"`).
///
/// ### Examples
/// ```rust
/// // use cans::content::do_replace;
/// use cans::do_replace;
///
/// let template = "<p>Hello, {{name}}! Welcome to {{place}}.</p>";
/// let result = do_replace!(template, name = "Dear", place = "CANS Template");
/// assert_eq!(result, "<p>Hello, Dear! Welcome to CANS Template.</p>");
/// ```
/// 
/// This macro can be used to update any content dynamically by replacing placeholders with actual values.
///
/// #### Example: Creating a new macro using `do_replace!`
///
/// Macros for specific content types can be defined by wrapping `do_replace!`.
/// For example, to create a macro for CSS content:
/// ```rust
/// // use cans::content::do_replace;
/// use cans::do_replace;
/// 
/// #[macro_export]
/// macro_rules! do_html {
///     ($content:expr, $($key:ident = $val:expr),*) => {
///         $crate::do_replace!($content, $($key = $val),*)
///     };
/// }
/// ```
/// Now, `do_css!` can be used to easily replace placeholders in CSS templates.
/// 
/// <small>End Doc</small>
#[macro_export]
macro_rules! do_replace {
    // Accept a content block and key-value pairs for substitution
    ($content:expr, $($key:ident = $val:expr),*) => {{
        let mut content_string = $content.to_string();

        // Iterate over all key-value pairs and replace placeholders
        $(
            // Replace placeholders like `{{name}}` in the content string
            let val = format!("{}", $val);
            content_string = content_string.replace(&format!("{{{{{}}}}}", stringify!($key)), &val);
        )*

        // Return the processed content string
        content_string
    }};
}

/// ### do_html!($content, $key, and $val)
///
/// Macro Rules
///
/// The `do_html` macro takes HTML string content along with key-value pairs and replaces placeholders in the HTML (formatted as `{{key}}`) with the corresponding values.
/// It returns the processed HTML string with the substitutions applied.
///
/// ### Parameters
/// - `$content`: The HTML string content containing placeholders for substitution (e.g., `"<p>Hello, {{name}}!</p>"`).
/// - `$key`: The identifier for each placeholder in the HTML (e.g., `name`).
/// - `$val`: The value that replaces the corresponding placeholder in the HTML (e.g., `"Alice"`).
///
/// ### Examples
/// ```rust
/// // use cans::content::do_html;
/// use cans::do_html;
///
/// let template = "<p>Hello, {{name}}! Welcome to {{place}}.</p>";
/// let result = do_html!(template, name = "Dear", place = "CANS Template");
/// assert_eq!(result, "<p>Hello, Dear! Welcome to CANS Template.</p>");
/// ```
/// <small>End Doc</small>
#[macro_export]
macro_rules! do_html {
    ($content:expr, $($key:ident = $val:expr),*) => {
        $crate::do_replace!($content, $($key = $val),*)
    };
}

/// ### do_xml!($content, $key, and $val)
///
/// Macro Rules
///
/// The `do_xml` macro takes XML string content along with key-value pairs and replaces placeholders in the XML (formatted as `{{key}}`) with the corresponding values.
/// It returns the processed XML string with the substitutions applied.
///
/// ### Parameters
/// - `$content`: The XML string content containing placeholders for substitution (e.g., `"<p>Hello, {{name}}!</p>"`).
/// - `$key`: The identifier for each placeholder in the XML (e.g., `name`).
/// - `$val`: The value that replaces the corresponding placeholder in the XML (e.g., `"Alice"`).
///
/// ### Examples
/// ```rust
/// // use cans::content::do_xml;
/// use cans::do_xml;
///
/// let xml_content = "<note><to>{{recipient}}</to></note>";
/// let xml_result = do_xml!(xml_content, recipient = "Ahmed");
/// assert_eq!(xml_result, "<note><to>Ahmed</to></note>");
/// ```
/// <small>End Doc</small>
#[macro_export]
macro_rules! do_xml {
    ($content:expr, $($key:ident = $val:expr),*) => {
        $crate::do_replace!($content, $($key = $val),*)
    };
}

/// ### do_json!($content, $key1 = $val1, $key2 = $val2, ...)
///
/// Macro Rules
///
/// The `do_json` macro takes a JSON string with placeholders and replaces them with provided values.
/// It is an example of creating a specific macro wrapper for JSON content using `do_replace!`.
///
/// ### Parameters
/// - `$content`: The JSON string containing placeholders (e.g., `"{\"name\": \"{{name}}\"}"`).
/// - `$key`: The placeholder identifier (e.g., `name`).
/// - `$val`: The value to replace the placeholder (e.g., `"Ahmed"`).
/// 
//// ### Examples
/// 
/// ```rust
/// // use cans::content::do_json;
/// use cans::do_json;
///
/// let json_content = r##"{"greeting": "{{greeting}}", "name": "{{name}}"}"##;
/// let json_result = do_json!(json_content, greeting = "Hi", name = "Ahmed");
/// assert_eq!(json_result, r##"{"greeting": "Hi", "name": "Ahmed"}"##);
/// ```
/// 
/// <small>End Doc</small>
#[macro_export]
macro_rules! do_json {
    ($content:expr, $($key:ident = $val:expr),*) => {
        $crate::do_replace!($content, $($key = $val),*)
    };
}