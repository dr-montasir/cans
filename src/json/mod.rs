pub use crate::do_json;

use std::any::Any;
use std::collections::HashMap;

/// A struct to parse and manage JSON-like key-value pairs.
///
/// `ParseJson` allows for the storage of key-value pairs, where values can be of different types.
/// The struct provides functionality to set, get, delete, and print these attributes.
pub struct ParseJson {
    attributes: HashMap<String, Box<dyn Any>>, // Use 'Box<dyn Any>' to allow for different types
}

#[allow(dead_code)]
impl ParseJson {
    /// Creates a new instance of `ParseJson`.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let parser = ParseJson::new();
    /// ```
    pub fn new() -> Self {
        ParseJson {
            attributes: HashMap::new(), // Initialize fields
        }
    }

    /// Sets multiple attributes using a JSON string.
    ///
    /// The method assumes a simplified JSON format and parses it to extract key-value pairs.
    /// Any invalid pairs will be skipped.
    ///
    /// ### Arguments
    ///
    /// * `json_string` - A string representing JSON data.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.set_all(r#"{ "key1": 42, "key2": "hello", "key3": true }"#);
    /// ```
    pub fn set_all(&mut self, json_string: &str) {
        // A very simple parsing assuming format: "key": value
        let cleaned_input = json_string
            .trim()
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap();
        for pair in cleaned_input.split(',') {
            let parts: Vec<&str> = pair.trim().split(':').collect();
            if parts.len() != 2 {
                continue; // Skip invalid pairs
            }
            let key = parts[0].trim().trim_matches('"').to_string();
            let value = parts[1].trim();

            // Determine type and set attributes accordingly
            if let Ok(num) = value.parse::<u32>() {
                self.set(key, num);
            } else if let Ok(num) = value.parse::<f64>() {
                self.set(key, num);
            } else if value == "true" {
                self.set(key, true);
            } else if value == "false" {
                self.set(key, false);
            } else if value.starts_with('"') && value.ends_with('"') {
                let str_value = value.trim_matches('\"').to_string();
                self.set(key, str_value);
            } else {
                self.set(key, "(unsupported type)"); // Handle unsupported types
            }
        }
    }

    /// Sets a single attribute.
    ///
    /// The type of the attribute is determined at compile time, allowing for different types of values
    /// to be stored. The value is boxed to support dynamic typing.
    ///
    /// ### Arguments
    ///
    /// * `key` - A string representing the key for the attribute.
    /// * `value` - The value to be associated with the key.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.set("key1".to_string(), 42);
    /// ```
    pub fn set<T: 'static>(&mut self, key: String, value: T) {
        self.attributes.insert(key, Box::new(value));
    }

    /// Gets a single attribute by key.
    ///
    /// Returns an `Option` which will be `Some(&T)` if the attribute exists and is of the requested type,
    /// or `None` if it does not exist or the type does not match.
    ///
    /// ### Arguments
    ///
    /// * `key` - A string slice representing the key for the attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.set("key1".to_string(), 42);
    /// if let Some(value) = parser.get::<u32>("key1") {
    ///     println!("Found: {}", value);
    /// }
    /// ```
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.attributes.get(key)?.downcast_ref::<T>()
    }

    /// Returns all attributes as a vector of tuples (key, value).
    ///
    /// The value is converted to a string representation based on its type.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.set("key1".to_string(), 42);
    /// let all_attributes = parser.get_all();
    /// ```
    pub fn get_all(&self) -> Vec<(String, String)> {
        self.attributes
            .iter()
            .map(|(k, v)| {
                let value_str = if let Some(v) = v.downcast_ref::<String>() {
                    v.clone()
                } else if let Some(v) = v.downcast_ref::<u32>() {
                    v.to_string()
                } else if let Some(v) = v.downcast_ref::<f64>() {
                    v.to_string()
                } else if let Some(v) = v.downcast_ref::<bool>() {
                    v.to_string()
                } else {
                    "(unknown type)".to_string()
                };
                (k.clone(), value_str)
            })
            .collect()
    }

    /// Prints all attributes to the standard output.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.set("key1".to_string(), 42);
    /// parser.print_all();
    /// ```
    pub fn print_all(&self) {
        for (key, value) in self.get_all() {
            println!("{}: {}", key, value);
        }
    }

    /// Prints a single attribute by key to the standard output.
    ///
    /// If the attribute does not exist, a message indicating that it was not found will be printed.
    ///
    /// ### Arguments
    ///
    /// * `key` - A string slice representing the key for the attribute.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.set("key1".to_string(), "hello");
    /// parser.print("key1");
    /// ```
    pub fn print(&self, key: &str) {
        if let Some(value) = self.attributes.get(key) {
            if let Some(v) = value.downcast_ref::<String>() {
                println!("{}: {}", key, v);
            } else if let Some(v) = value.downcast_ref::<u32>() {
                println!("{}: {}", key, v);
            } else if let Some(v) = value.downcast_ref::<f64>() {
                println!("{}: {}", key, v);
            } else if let Some(v) = value.downcast_ref::<bool>() {
                println!("{}: {}", key, v);
            } else {
                println!("{}: (unknown type)", key); // Handle unknown types
            }
        } else {
            println!("Attribute '{}' not found", key);
        }
    }

    /// Partially updates attributes with new values.
    ///
    /// This method will insert or update attributes based on the provided `HashMap`.
    ///
    /// ### Arguments
    ///
    /// * `updates` - A hashmap containing the new key-value pairs to update.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    /// use std::any::Any;
    /// use std::collections::HashMap;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.set("key1".to_string(), 42);
    /// let mut updates = HashMap::new();
    /// updates.insert("key1".to_string(), Box::new(100) as Box<dyn Any>);
    /// parser.patch(updates);
    /// ```
    pub fn patch(&mut self, updates: HashMap<String, Box<dyn Any>>) {
        for (key, value) in updates {
            self.attributes.insert(key, value);
        }
    }

    /// Replaces all attributes with new attributes from a given `HashMap`.
    ///
    /// ### Arguments
    ///
    /// * `new_attributes` - A hashmap containing the new key-value pairs to replace existing attributes.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    /// use std::any::Any;
    /// use std::collections::HashMap;
    ///
    /// let mut parser = ParseJson::new();
    /// let mut new_attrs = HashMap::new();
    /// new_attrs.insert("key2".to_string(), Box::new("world") as Box<dyn Any>);
    /// parser.put(new_attrs);
    /// ```
    pub fn put(&mut self, new_attributes: HashMap<String, Box<dyn Any>>) {
        self.attributes = new_attributes;
    }

    /// Deletes a specific attribute by key.
    ///
    /// ### Arguments
    ///
    /// * `key` - A string slice representing the key for the attribute to remove.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.set("key1".to_string(), 42);
    /// parser.delete("key1");
    /// ```
    pub fn delete(&mut self, key: &str) {
        self.attributes.remove(key);
    }

    /// Deletes all attributes stored in `ParseJson`.
    ///
    /// ### Examples
    ///
    /// ```
    /// use cans::json::ParseJson;
    ///
    /// let mut parser = ParseJson::new();
    /// parser.delete_all();
    /// ```
    pub fn delete_all(&mut self) {
        self.attributes.clear();
    }
}

// Implement Debug for ParseJson to include full attributes
impl std::fmt::Debug for ParseJson {
    /// Formats the output for `Debug` to represent stored attributes.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;

        for (i, (k, v)) in self.attributes.iter().enumerate() {
            // Write key
            write!(f, "\"{}\": ", k)?;

            // Determine the type and write the value
            if let Some(v) = v.downcast_ref::<String>() {
                write!(f, "\"{}\"", v)?;
            } else if let Some(v) = v.downcast_ref::<u32>() {
                write!(f, "{}", v)?;
            } else if let Some(v) = v.downcast_ref::<f64>() {
                write!(f, "{}", v)?;
            } else if let Some(v) = v.downcast_ref::<bool>() {
                write!(f, "{}", v)?;
            } else {
                write!(f, "\"(unknown type)\"")?; // Handle unknown types
            }

            if i < self.attributes.len() - 1 {
                write!(f, ", ")?; // Add comma for all but the last item
            }
        }

        f.write_str("}")?;
        Ok(())
    }
}
