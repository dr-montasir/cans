use std::collections::HashMap;

/// A simple country manager struct to hold and manage country details.
///
/// # Example
///
/// ```
/// use cans::world::Country;
///
/// let mut country_manager = Country::new();
/// println!("Current Data: {:?}", country_manager.retrieve());
///
/// // Insert a new country using Vec<&str>
/// country_manager.insert_one("FR", vec!["🇫🇷", "33", "France", "EUR"]);
///
/// // Insert multiple countries using Vec<&str>
/// country_manager.insert_many(vec![
///     ("IT", vec!["🇮🇹", "39", "Italy", "EUR"]),
///     ("DE", vec!["🇩🇪", "49", "Germany", "EUR"]),
/// ]);
///
/// println!("After Insertions: {:?}", country_manager.retrieve());
///
/// // Sort and show the data in ascending order
/// let sorted_data_asc = country_manager.sort_asc();
/// println!("Sorted Data (Ascending): {:?}", sorted_data_asc);
///
/// // Sort and show the data in descending order
/// let sorted_data_desc = country_manager.sort_desc();
/// println!("Sorted Data (Descending): {:?}", sorted_data_desc);
///
/// // Delete a specific country
/// country_manager.delete_one("CA");
/// println!("After Deleting CA: {:?}", country_manager.retrieve());
///
/// // Delete all countries
/// country_manager.delete_all();
/// println!("After Deleting All: {:?}", country_manager.retrieve());
/// ```
#[derive(Debug)]
pub struct Details {
    pub flag: String,     // Emoji flag of the country
    pub code: String,     // International dialing code
    pub name: String,     // Name of the country
    pub currency: String, // Currency used in the country
}

/// Struct to manage a collection of countries.
pub struct Country {
    data: HashMap<String, Details>, // HashMap to store details of countries keyed by their short names
}

impl Country {
    /// Creates a new Country instance with some initial example entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let country_manager = Country::new();
    /// assert!(country_manager.retrieve().len() > 0); // Assert that there are initial entries
    /// ```
    pub fn new() -> Self {
        let mut data = HashMap::new();

        // Example entries
        let entries = [
            ("BH", "🇧🇭", "973", "Bahrain", "BHD"),
            ("CA", "🇨🇦", "1", "Canada", "CAD"),
            ("DZ", "🇩🇿", "213", "Algeria", "DZD"),
            ("SA", "🇸🇦", "966", "Saudi Arabia", "SAR"),
            ("SD", "🇸🇩", "249", "Sudan", "SDG"),
            ("GB", "🇬🇧", "44", "United Kingdom", "GBP"),
            ("UK", "🇬🇧", "44", "United Kingdom", "GBP"),
            ("US", "🇺🇸", "1", "United States", "USD"),
        ];

        for &(short_name, flag, code, country_name, currency) in &entries {
            data.insert(
                short_name.to_string(),
                Details {
                    flag: flag.to_string(),
                    code: code.to_string(),
                    name: country_name.to_string(),
                    currency: currency.to_string(),
                },
            );
        }

        Country { data }
    }

    /// Inserts a single country into the collection.
    ///
    /// # Arguments
    ///
    /// * `short_name` - A string slice that holds the short name of the country.
    /// * `details` - A vector containing details of the country: [flag, code, name, currency].
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let mut country_manager = Country::new();
    /// country_manager.insert_one("FR", vec!["🇫🇷", "33", "France", "EUR"]);
    /// assert_eq!(country_manager.retrieve().len(), 9); // Check if the country was added
    /// ```
    pub fn insert_one(&mut self, short_name: &str, details: Vec<&str>) {
        let details_struct = Details {
            flag: details[0].to_string(),
            code: details[1].to_string(),
            name: details[2].to_string(),
            currency: details[3].to_string(),
        };
        self.data.insert(short_name.to_string(), details_struct);
    }

    /// Inserts multiple countries into the collection.
    ///
    /// # Arguments
    ///
    /// * `entries` - A vector of tuples where each tuple contains
    ///   a short name and a vector of country details.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let mut country_manager = Country::new();
    /// country_manager.insert_many(vec![
    ///     ("IT", vec!["🇮🇹", "39", "Italy", "EUR"]),
    ///     ("DE", vec!["🇩🇪", "49", "Germany", "EUR"]),
    /// ]);
    /// assert_eq!(country_manager.retrieve().len(), 10); // Check if both countries were added
    /// ```
    pub fn insert_many(&mut self, entries: Vec<(&str, Vec<&str>)>) {
        for (short_name, details) in entries {
            self.insert_one(short_name, details);
        }
    }

    /// Deletes a single country from the collection by its short name.
    ///
    /// # Arguments
    ///
    /// * `short_name` - A string slice that holds the short name of the country to delete.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let mut country_manager = Country::new();
    /// country_manager.delete_one("CA");
    /// assert!(country_manager.retrieve().contains_key("CA") == false); // Assert that CA is deleted
    /// ```
    pub fn delete_one(&mut self, short_name: &str) {
        self.data.remove(short_name);
    }

    /// Deletes all countries from the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let mut country_manager = Country::new();
    /// country_manager.delete_all();
    /// assert_eq!(country_manager.retrieve().len(), 0); // Check if all countries were deleted
    /// ```
    pub fn delete_all(&mut self) {
        self.data.clear();
    }

    /// Retrieves the current collection of countries.
    ///
    /// # Returns
    ///
    /// A reference to the HashMap containing country details.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let country_manager = Country::new();
    /// let data = country_manager.retrieve();
    /// assert!(data.len() > 0); // There should be initial data
    /// ```
    pub fn retrieve(&self) -> &HashMap<String, Details> {
        &self.data
    }

    /// Sorts the countries in ascending order, placing the United States entry last.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing references to the sorted country data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let country_manager = Country::new();
    /// let sorted_data = country_manager.sort_asc();
    /// assert!(sorted_data.last().unwrap().0 == "US"); // Check if US is last
    /// ```
    pub fn sort_asc(&self) -> Vec<(&String, &Details)> {
        let mut sorted: Vec<_> = self.data.iter().collect();

        sorted.sort_by(|a, b| {
            if a.0 == "US" {
                std::cmp::Ordering::Greater // Place "US" at the end
            } else if b.0 == "US" {
                std::cmp::Ordering::Less // Place "US" at the end
            } else {
                a.0.cmp(b.0) // Regular sorting for other entries
            }
        });

        sorted
    }

    /// Sorts the countries in descending order, placing the United States entry last.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing references to the sorted country data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let country_manager = Country::new();
    /// let sorted_data = country_manager.sort_desc();
    /// assert!(sorted_data.last().unwrap().0 == "US"); // Check if US is last
    /// ```
    pub fn sort_desc(&self) -> Vec<(&String, &Details)> {
        let mut sorted: Vec<_> = self.data.iter().collect();

        sorted.sort_by(|a, b| {
            if a.0 == "US" {
                std::cmp::Ordering::Greater // Place "US" at the end
            } else if b.0 == "US" {
                std::cmp::Ordering::Less // Place "US" at the end
            } else {
                b.0.cmp(a.0) // Sort in descending order for other entries
            }
        });

        sorted
    }
}
