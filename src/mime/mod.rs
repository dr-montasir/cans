use std::collections::HashMap;

/// ### set_mime_types
///
/// Initializes a HashMap with common file extensions and their corresponding MIME types.
///
/// The `set_mime_types` function initializes a collection of MIME types and their
/// associated file extensions, returning this mapping as a HashMap. It serves as
/// the starting point for managing MIME types in applications, providing a comprehensive
/// list of common file extensions and their recognized types.
///
/// ### Returns
/// 
/// - Returns a `HashMap<String, String>` containing common file extensions as keys
///   and their corresponding MIME types as values.
///
/// Examples
///
/// ```rust
/// use cans::mime::{display_mime_types, set_mime_types};
///
/// fn main() {
///    // Create a new HashMap and initialize it with default MIME types
///    let mime_types = set_mime_types();
///    
///    println!("Initial MIME types:");
///    display_mime_types(&mime_types);
/// }
/// ```
///
/// ### Output
///
/// ```terminal
/// Initial MIME types:
/// Extension: application - MIME Type: application/octet-stream
/// Extension: mkv - MIME Type: video/x-matroska
/// Extension: mpg - MIME Type: video/mpeg
/// Extension: mov - MIME Type: video/quicktime
/// Extension: gz - MIME Type: application/gzip
/// Extension: ics - MIME Type: text/calendar
/// Extension: txt - MIME Type: text/plain
/// ...
/// ```
///
/// ### Usage Context
/// 
/// This function is typically called at the start of an application to establish
/// default MIME type mappings, ensuring that subsequent operations on file types
/// can utilize a reliable set of MIME definitions.
///
/// <small>End Fun Doc</small>
pub fn set_mime_types() -> HashMap<String, String> {
    let mime_types: HashMap<String, String> = [
        ("html", "text/html"),
        ("ejs", "text/html"),
        ("css", "text/css"),
        ("scss", "text/scss"),
        ("less", "text/css"),
        ("csv", "text/csv"),
        ("rs", "text/plain"),
        ("plain", "text/plain"),
        ("txt", "text/plain"),
        ("markdown", "text/markdown"),
        ("md", "text/markdown"),
        ("xml", "text/xml"),
        ("yaml", "text/yaml"),
        ("wat", "text/wat"),
        ("py", "text/x-python"),
        ("log", "text/plain"),
        ("ics", "text/calendar"),
        ("vcard", "text/vcard"),
        ("php", "text/x-php"),
        ("conf", "text/plain"),
        ("init", "text/plain"),
        ("png", "image/png"),
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("gif", "image/gif"),
        ("svg", "image/svg+xml"),
        ("svgz", "image/svg+xml"),
        ("bmp", "image/bmp"),
        ("tiff", "image/tiff"),
        ("ico", "image/x-icon"),
        ("webp", "image/webp"),
        ("woff", "font/woff"),
        ("woff2", "font/woff2"),
        ("ttf", "font/ttf"),
        ("otf", "font/otf"),
        ("eot", "application/vnd.ms-fontobject"),
        ("js", "application/javascript"),
        ("ts", "application/typescript"),
        ("xml", "application/xml"),
        ("json", "application/json"),
        ("geojson", "application/geo+json"),
        ("map", "application/json-patch+json"),
        ("jsonld", "application/ld+json"),
        ("binary", "application/octet-stream"),
        ("tar", "application/x-tar"),
        ("so", "application/octet-stream"),
        ("so", "application/x-sharedlib"),
        ("deb", "application/vnd.debian.binary-package"),
        ("bash", "application/x-shellscript"),
        ("php", "application/x-httpd-php"),
        ("db", "application/x-database"),
        ("sql", "application/sql"),
        ("pdf", "application/pdf"),
        ("zip", "application/zip"),
        ("gz", "application/gzip"),
        ("epub", "application/epub+zip"),
        ("toml", "application/toml"),
        ("xhtml", "application/xhtml+xml"),
        ("rss", "application/rss+xml"),
        ("atom", "application/atom+xml"),
        ("graphql", "application/graphql"),
        ("form", "application/x-www-form-urlencoded"),
        ("srt", "application/x-subrip"),
        ("rtf", "application/rtf"),
        ("kml", "application/vnd.google-earth.kml+xml"),
        ("kmz", "application/vnd.google-earth.kmz"),
        ("sh", "application/x-sh"),
        ("bin", "application/octet-stream"),
        ("exe", "application/octet-stream"),
        ("dll", "application/octet-stream"),
        ("class", "application/java-vm"),
        ("jar", "application/java-archive"),
        ("wasm", "application/wasm"),
        ("wat", "application/wasm"),
        ("conf", "application/x-configuration"),
        ("init", "application/x-initialization"),
        ("pem", "application/x-x509-ca-cert"),
        ("p12", "application/x-pkcs12"),
        ("p7b", "application/x-pkcs7-certificates"),
        ("p7c", "application/pkcs7-mime"),
        ("pot", "application/vnd.ms-powerpoint"),
        ("doc", "application/msword"),
        (
            "docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ),
        ("xls", "application/vnd.ms-excel"),
        (
            "xlsx",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
        ("ppt", "application/vnd.ms-powerpoint"),
        (
            "pptx",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        ),
        (
            "dotx",
            "application/vnd.openxmlformats-officedocument.wordtemplate",
        ),
        ("xla", "application/vnd.ms-excel"),
        ("xlt", "application/vnd.ms-excel"),
        (
            "xltx",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
        ),
        ("bat", "application/bat"),
        ("pub", "application/x-mspublisher"),
        ("xps", "application/vnd.ms-xpsdocument"),
        ("msg", "application/vnd.ms-outlook"),
        ("dot", "application/msword"),
        ("wps", "application/vnd.ms-works"),
        ("odt", "application/vnd.oasis.opendocument.text"),
        ("ods", "application/vnd.oasis.opendocument.spreadsheet"),
        ("apk", "application/vnd.android.package-archive"),
        ("iot", "application/x-iot-file"),
        ("iot", "application/x-iot-data"),
        ("application", "application/octet-stream"),
        ("wav", "audio/wav"),
        ("mp3", "audio/mpeg"),
        ("aac", "audio/aac"),
        ("ogg", "audio/ogg"),
        ("m4a", "audio/mp4a-latm"),
        ("flac", "audio/flac"),
        ("midi", "audio/midi"),
        ("mid", "audio/midi"),
        ("mp4", "video/mp4"),
        ("webm", "video/webm"),
        ("mov", "video/quicktime"),
        ("avi", "video/x-msvideo"),
        ("flv", "video/x-flv"),
        ("mkv", "video/x-matroska"),
        ("mpg", "video/mpeg"),
        ("mpeg", "video/mpeg"),
        ("mpa", "video/x-mpeg"),
    ]
    .iter()
    .map(|&(ext, mime)| (ext.to_string(), mime.to_string()))
    .collect();

    mime_types
}

/// ### manage_mime_types
///
/// A helper function to manage MIME types by converting an array of tuples into a HashMap.
///
/// This function takes a slice of tuples containing file extensions and their
/// corresponding MIME types as input and returns a HashMap. It facilitates
/// the creation of various MIME type mappings in a consistent manner, making it
/// easier to insert or remove MIME type mappings as needed.
///
/// ### Parameters
/// 
/// - `mime_types`: A slice of tuples, where each tuple contains a file extension
///   and its respective MIME type.
///
/// ### Returns
/// 
/// - Returns a `HashMap<String, String>` containing the provided file extensions
///   as keys and their corresponding MIME types as values.
///
/// <small>End Fun Doc</small>
pub fn manage_mime_types(mime_types: &[(&str, &str)]) -> HashMap<String, String> {
    mime_types
        .iter()
        .map(|&(ext, mime)| (ext.to_string(), mime.to_string()))
        .collect()
}

/// ### insert_mime_type
///
/// Inserts a single MIME type into an existing HashMap for the given file extension.
///
/// The `insert_mime_type` function allows users to extend the existing list of MIME
/// types dynamically. By providing a file extension and its corresponding MIME type,
/// this function can be used to add new entries as needed.
///
/// ### Parameters
/// 
/// - `mime_types`: A mutable reference to a `HashMap<String, String>` of MIME types.
/// - `extension`: A string slice (`&str`) representing the file extension to be added.
/// - `mime_type`: A string slice (`&str`) representing the corresponding MIME type.
///
/// ### Examples
/// 
/// ```rust
/// use cans::mime::{
///     display_mime_types,
///     insert_mime_type,
///     set_mime_types,
/// };
///
/// fn main() {
///    // Create a new HashMap and initialize it with default MIME types
///    let mut mime_types = set_mime_types();
///
///    // Insert a new MIME type
///    insert_mime_type(&mut mime_types, "txt", "text/plain");
///    println!("\nAfter inserting .txt MIME type:");
///    display_mime_types(&mime_types);
/// }
/// ```
///
/// ### Output
///
/// ```terminal
/// After inserting .txt MIME type:
/// Extension: log - MIME Type: text/plain
/// ...
/// Extension: txt - MIME Type: text/plain
/// ...
/// ```
///
/// ### Usage Context
/// 
/// This function can be useful when dealing with custom file types or when extending
/// the MIME type definitions beyond the defaults provided in `set_mime_types`.
///
/// <small>End Fun Doc</small>
pub fn insert_mime_type(
    mime_types: &mut HashMap<String, String>,
    extension: &str,
    mime_type: &str,
) {
    mime_types.insert(extension.to_string(), mime_type.to_string());
}

/// ### remove_mime_type
///
/// Removes a MIME type for a specified file extension from the HashMap.
///
/// The `remove_mime_type` function is utilized to delete existing MIME definitions
/// based on file extensions. This helps maintain an accurate and relevant collection
/// of MIME type definitions as the application evolves.
///
/// ### Parameters
/// 
/// - `mime_types`: A mutable reference to a `HashMap<String, String>` of MIME types.
/// - `extension`: A string slice (`&str`) representing the file extension to remove.
///
/// ### Examples
/// 
/// ```rust
/// use cans::mime::{
///     remove_mime_type,
///     set_mime_types,
/// };
///
/// let mut mime_types = set_mime_types();
/// remove_mime_type(&mut mime_types, "html");
/// assert_eq!(mime_types.get("html"), None);
/// ```
/// 
/// ```rust
/// use cans::mime::{
///     display_mime_types,
///     remove_mime_type,
///     set_mime_types,
/// };
///
/// fn main() {
///    // Create a new HashMap and initialize it with default MIME types
///    let mut mime_types = set_mime_types();
///
///    // Remove an existing MIME type
///    remove_mime_type(&mut mime_types, "jpg");
///    println!("\nAfter removing .jpg MIME type:");
///    display_mime_types(&mime_types);
/// }
/// ```
/// 
/// ### Usage Context
/// 
/// This function is essential for scenarios where certain MIME types need to be
/// deprecated or removed from the active listings, ensuring that the application
/// stays up-to-date with the current file typing needs.
///
/// <small>End Fun Doc</small>
pub fn remove_mime_type(mime_types: &mut HashMap<String, String>, extension: &str) {
    mime_types.remove(extension);
}

/// ### insert_mime_types
///
/// Inserts entries from a given HashMap of MIME types into an existing HashMap.
///
/// The `insert_mime_types` function takes another `HashMap` containing MIME types
/// and merges its entries into the provided existing `HashMap`. This allows for
/// extensibility and flexibility when managing MIME types, facilitating the
/// addition of multiple entries at once.
///
/// ### Parameters
///
/// - `mime_types`: A mutable reference to an existing `HashMap<String, String>`
///   that will receive new MIME type entries.
/// - `mime_types_map`: A `HashMap<String, String>` containing MIME type entries
///   to be inserted into the existing `mime_types` HashMap.
/// 
/// ```rust
/// use cans::mime::{
///     display_mime_types,
///     insert_mime_types,
///     manage_mime_types,
///     set_mime_types,
/// };
/// 
/// fn main() {
///      // Create a new HashMap and initialize it with default MIME types
///      let mut mime_types = set_mime_types();
/// 
///      // Display the default count of MIME types
///      println!("The default length of MIME types: {}", mime_types.len()); // Example output: 113
/// 
///      // Define an array of tuples for both already included and new MIME types
///      let included_mime_types = manage_mime_types(&[
///          // Already included MIME types
///          ("html", "text/html"),
///          ("css", "text/css"),
///          ("js", "application/javascript"),
///          ("svg", "image/svg+xml"),
///          ("gif", "image/gif"),
///          ("jpg", "image/jpeg"),
///          ("txt", "text/plain"),
///          ("json", "application/json"),
///          // New MIME type to include
///          ("some_ext", "some_mime"),
///      ]);
/// 
///      // Insert new MIME types into the existing HashMap
///      insert_mime_types(&mut mime_types, included_mime_types);
/// 
///      // Display the length after insertion
///      println!(
///          "The length after inserting MIME types: {}",
///          mime_types.len() // Example output: 114
///      );
/// 
///     // Display the updated MIME types
///     println!("\nAfter inserting MIME types:");
///     display_mime_types(&mime_types);
/// }
/// ```
///
/// ### Usage Context
/// 
/// This function is particularly useful when there is a need to bulk-add MIME types
/// from a predefined list or configuration. It simplifies the process of extending
/// the supported MIME types within an application, improving maintainability and
/// organization.
///
/// <small>End Fun Doc</small>
pub fn insert_mime_types(
    mime_types: &mut HashMap<String, String>,
    mime_types_map: HashMap<String, String>,
) {
    for (extension, mime_type) in mime_types_map {
        mime_types.insert(extension, mime_type);
    }
}

/// ### remove_mime_types
///
/// Removes multiple MIME types based on specified file extensions from the HashMap.
///
/// The `remove_mime_types` function allows for the bulk removal of existing MIME
/// definitions based on their associated file extensions. This facilitates effective
/// management of MIME type definitions, enabling applications to remain efficient
/// and relevant as they evolve.
///
/// ### Parameters
/// 
/// - `mime_types`: A mutable reference to a `HashMap<String, String>` containing
///   MIME types, from which entries will be removed.
/// - `mime_types_map`: A `HashMap<String, String>` where the keys represent
///   the file extensions to be removed from the existing `mime_types` HashMap.
///
/// ### Examples
/// 
/// ```rust
/// use cans::mime::{
///     display_mime_types,
///     manage_mime_types,
///     remove_mime_types,
///     set_mime_types,
/// };
///
/// fn main() {
///     // Create a new HashMap and initialize it with default MIME types
///     let mut mime_types = set_mime_types();
///
///     // Display the default count of MIME types
///     println!("The default length of MIME types: {}", mime_types.len()); // Example output: 113
///
///     // Define an array of tuples for both pre-existing and new MIME types
///     let excluded_mime_types = manage_mime_types(&[
///         // Pre-existing MIME types by default
///         ("html", "text/html"),
///         ("css", "text/css"),
///         ("js", "application/javascript"),
///         ("svg", "image/svg+xml"),
///         ("gif", "image/gif"),
///         ("jpg", "image/jpeg"),
///         ("txt", "text/plain"),
///         ("json", "application/json"),
///         // New MIME type that is not part of the default list
///         ("some_ext", "some_mime"),
///     ]);
///
///     // Attempt to remove MIME types from the existing HashMap
///     remove_mime_types(&mut mime_types, excluded_mime_types);
///
///     // Display the length after removal
///     println!(
///         "The length after removing MIME types: {}",
///         mime_types.len() // Example output: 105
///     );
///
///     // Display the updated MIME types
///     println!("\nAfter removing MIME types:");
///     display_mime_types(&mime_types);
/// }
/// ```
///
/// ### Usage Context
/// 
/// This function is particularly useful in scenarios where multiple MIME types
/// need to be deprecated or removed in a single operation. It helps maintain an
/// organized and relevant collection of MIME type definitions, ensuring application
/// consistency and efficiency.
///
/// <small>End Fun Doc</small>
pub fn remove_mime_types(
    mime_types: &mut HashMap<String, String>,
    mime_types_map: HashMap<String, String>,
) {
    for extension in mime_types_map.keys() {
        mime_types.remove(extension);
    }
}

/// ### display_mime_types
///
/// Prints all the current MIME types stored in a `HashMap` to the console.
///
/// This function iterates over the provided `HashMap` containing file extensions as keys
/// and their corresponding MIME types as values. For each key-value pair, it prints the
/// extension and the associated MIME type in a human-readable format.
///
/// ### Parameters
/// 
/// - `mime_types`: A reference to a `HashMap<String, String>`, which contains
///   mappings of file extensions to their corresponding MIME types.
///
/// ### Examples
///
/// ```rust
/// use cans::mime::{display_mime_types, set_mime_types};
///
/// fn main() {
///     // Initialize MIME types
///     let mime_types = set_mime_types();
///
///     // Display all MIME types
///     display_mime_types(&mime_types);
///     // Output:
///     // Extension: html - MIME Type: text/html
///     // Extension: css - MIME Type: text/css
///     // Extension: js - MIME Type: application/javascript
///     // ...
/// }
/// ```
///
/// ### Notes
/// 
/// - Ensure that the `mime_types` HashMap is not empty to see any output.
/// - This function is intended for display purposes and does not modify the input HashMap.
///
/// <small>End Fun Doc</small>
pub fn display_mime_types(mime_types: &HashMap<String, String>) {
    for (extension, mime_type) in mime_types {
        println!("Extension: {} - MIME Type: {}", extension, mime_type);
    }
}

/// ### default_mime_types
///
/// Displays the default set of MIME types.
///
/// This function retrieves the default MIME types by calling the `set_mime_types()` function,
/// which returns a HashMap containing extensions and their corresponding MIME types.
/// It then prints each extension alongside its MIME type in a formatted manner.
///
/// ### Example
///
/// ```rust
/// use cans::mime::default_mime_types;
///
/// default_mime_types();
/// ```
///
/// ### Sample Output
///
/// ```terminal
/// Default MIME types:
/// Extension: html - MIME Type: text/html
/// Extension: css - MIME Type: text/css
/// Extension: js - MIME Type: application/javascript
/// Extension: svg - MIME Type: image/svg+xml
/// ...
/// ```
///
/// ### Notes
///
/// - The function does not take any parameters and does not return any values.
/// - It is primarily intended for debugging and informational purposes, helping users
///   understand the default MIME types recognized by the application.
///
/// <small>End Fun Doc</small>
pub fn default_mime_types() {
    println!("\nDefault MIME types:");
    let default_mime_types = set_mime_types();
    for (extension, mime_type) in &default_mime_types {
        println!("Extension: {} - MIME Type: {}", extension, mime_type);
    }
}
