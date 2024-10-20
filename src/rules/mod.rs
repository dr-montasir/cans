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

/// ### do_json!($json_string, key = $key, val = $val)
///
/// Macro Rules
///
/// A macro for constructing JSON-like strings with placeholders for dynamic values.
///
/// This macro allows you to create a JSON string from a template with placeholders
/// for values that are formatted based on their type. It supports strings, numbers,
/// booleans, and arrays. Placeholders are created using `{{key}}` syntax in the
/// initial `json_string`, which can then be replaced with actual values at runtime.
///
/// ### Examples
///
/// ```rust
/// use cans::do_json;
///
/// fn main() {
///     let name = "Alice";
///     let age = 30;
///     let is_student = false;
///     let json_string = do_json!(r#"{"name": "{{name}}", "age": {{age}}, "is_student": {{is_student}}" }"#,
///         name = name,
///         age = age,
///         is_student = is_student
///     );
///     println!("{}", json_string);
///     // Output: {"name": "Alice", "age": 30, "is_student": false}
/// }
/// ```
///
/// #### Using Arrays
///
/// ```rust
/// use cans::do_json;
///
/// fn main() {
///     let hobbies = vec!["reading", "archery", "hiking"];
///     let json_hobbies = do_json!(r#"{ "hobbies": {{hobbies}} }"#, hobbies = &hobbies);
///     
///     println!("{}", json_hobbies);
///     // Output: { "hobbies": ["reading", "archery", "hiking"] }
/// }
/// ```
///
/// ### Patterns
///
/// 1. **Raw JSON String with Placeholders**:
///    - **Input**: `$json_string` is a string literal containing placeholders like `{{key}}`.
///    - **Output**: Returns a JSON string with placeholders replaced by corresponding values.
///
/// 2. **Handling Strings**:
///    - **Input**: `$value:expr` as a string.
///    - **Output**: Formats the string correctly as a JSON string.
///
/// 3. **Handling Numbers**:
///    - **Input**: `$value:expr` as an integer or a float.
///    - **Output**: Converts the number to a string representation as-is.
///
/// 4. **Handling Booleans**:
///    - **Input**: `$value:expr` as a boolean.
///    - **Output**: Converts the boolean to "true" or "false".
///
/// 5. **Handling Arrays**:
///    - **Input**: `[$($value:expr),*]` for an array of values.
///    - **Output**: Returns a JSON array with values correctly formatted.
///
/// 6. **Handling Vectors**:
///    - **Input**: `$value:expr` as a Vec type.
///    - **Output**: Uses the Debug formatting to represent the elements in Vec.
///
/// ### Notes
/// - Ensure proper escaping for string literals within the template JSON string.
/// - This macro is flexible for dynamic creation of JSON-like structures, aiding in easier string manipulation while preparing data for output.
#[macro_export]
macro_rules! do_json {
    // Handle the raw JSON string with placeholders
    ($json_string:expr, $($key:ident = $value:expr),* $(,)?) => {{
        let mut json_string = $json_string.to_string();
        $(
            // Create a placeholder for the key
            let placeholder = format!("{{{{{}}}}}", stringify!($key));
            let value_str = do_json!($value);
            json_string = json_string.replace(&placeholder, &value_str);
        )*
        json_string
    }};

    // Handle strings
    ($value:expr) => {{
        format!(r#"{:?}"#, $value) // Correctly format strings as JSON strings
    }};

    // Handle numbers (integers and floats)
    ($value:expr) => {{
        $value.to_string() // Keep numbers as they are
    }};

    // Handle booleans
    ($value:expr) => {{
        if $value {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }};

    // Handle arrays (including Vec)
    ([$($value:expr),* $(,)?]) => {{
        let mut json_string = String::from("[");
        $(
            json_string.push_str(&do_json!($value));
            json_string.push(',');
        )*

        if json_string.len() > 1 {
            json_string.truncate(json_string.len() - 1); // Remove trailing comma
        }
        json_string.push(']');
        json_string
    }};

    // Handle Vec specifically
    ($value:expr) => {{
        // Use the Debug formatting for Vec types
        format!("{:?}", $value) // Use {:?} to format Vec
    }};
}
