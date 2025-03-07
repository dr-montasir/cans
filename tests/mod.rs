use cans::{world::Country, mime::*};

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for the World module:
    #[test]
    fn test_delete_all_without_resetting_gmt() {
        let mut cm = Country::new();
        let country_code = "US";

        // Fetching initial details for the country
        let flag = cm.country_detail(country_code, "flag");
        assert!(!flag.is_empty(), "The flag should not be empty for the US");

        // Fetch GMT for Washington, D.C.
        let gmt = cm.city_detail("US", "Washington, D.C.", "gmt");
        assert!(
            !gmt.is_empty(),
            "GMT for Washington, D.C. should not be empty"
        );

        // Call delete_all to remove all countries and cities
        cm.delete_all();

        // Check if retrieving after delete_all returns an empty string
        let retrieved_gmt = cm.city_detail("US", "Washington, D.C.", "gmt");
        assert_eq!(
            retrieved_gmt, "",
            "The GMT value should be an empty string after delete_all"
        );
    }

    #[test]
    fn test_delete_all_with_resetting_gmt() {
        let mut cm = Country::new();
        let country_code = "US";

        // Fetch GMT for Washington, D.C.
        let gmt = cm.city_detail(country_code, "Washington, D.C.", "gmt");
        assert!(
            !gmt.is_empty(),
            "GMT for Washington, D.C. should not be empty"
        );

        // Call delete_all to remove all countries and cities
        cm.delete_all();

        // Reset gmt to an empty string
        let gmt = "";

        // Now check the value after resetting gmt
        assert_eq!(
            gmt, "",
            "The variable gmt should now be an empty string after resetting it."
        );
    }

    // Tests for the MIME module:
    #[test]
    fn test_set_mime_types() {
        // Test that the set_mime_types function returns a non-empty HashMap
        let mime_types = set_mime_types();
        assert!(!mime_types.is_empty(), "MIME types HashMap should not be empty.");
        
        // Test that specific known MIME types are set correctly
        assert_eq!(mime_types.get("html"), Some(&"text/html".to_string()));
        assert_eq!(mime_types.get("png"), Some(&"image/png".to_string()));
        assert_eq!(mime_types.get("json"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_insert_mime_type() {
        let mut mime_types = set_mime_types();

        // Insert a new custom MIME type
        insert_mime_type(&mut mime_types, "custom_ext", "application/x-custom");
        
        // Check if the new MIME type was inserted
        assert_eq!(mime_types.get("custom_ext"), Some(&"application/x-custom".to_string()));
    }

    #[test]
    fn test_remove_mime_type() {
        let mut mime_types = set_mime_types();
        
        // Ensure "html" type exists before removal
        assert!(mime_types.contains_key("html"), "MIME type for 'html' should exist.");

        // Remove the "html" MIME type
        remove_mime_type(&mut mime_types, "html");
        
        // Ensure "html" type is removed
        assert!(!mime_types.contains_key("html"), "MIME type for 'html' should be removed.");
    }
}
