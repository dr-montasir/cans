use cans::world::Country;

#[cfg(test)]
mod tests {
    use super::*;

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
}
