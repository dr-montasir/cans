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
/// country_manager.insert_one("FR", vec!["🇫🇷", "33", "France", "EUR", "Paris"]);
///
/// // Insert multiple countries using Vec<&str>
/// country_manager.insert_many(vec![
///     ("IT", vec!["🇮🇹", "39", "Italy", "EUR", "Rome"]),
///     ("DE", vec!["🇩🇪", "49", "Germany", "EUR", "Berlin"]),
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
    pub capital: String,  // Capital of the country
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
            ("BH", "🇧🇭", "973", "Bahrain", "BHD", "Manama"),
            ("CA", "🇨🇦", "1", "Canada", "CAD", "Ottawa"),
            ("DZ", "🇩🇿", "213", "Algeria", "DZD", "Algiers"),
            ("SA", "🇸🇦", "966", "Saudi Arabia", "SAR", "Riyadh"),
            ("SD", "🇸🇩", "249", "Sudan", "SDG", "Khartoum"),
            ("GB", "🇬🇧", "44", "United Kingdom", "GBP", "London"),
            ("UK", "🇬🇧", "44", "United Kingdom", "GBP", "London"),
            ("US", "🇺🇸", "1", "United States", "USD", "Washington, D.C."),
        ];

        for &(short_name, flag, code, country_name, currency, capital) in &entries {
            data.insert(
                short_name.to_string(),
                Details {
                    flag: flag.to_string(),
                    code: code.to_string(),
                    name: country_name.to_string(),
                    currency: currency.to_string(),
                    capital: capital.to_string(),
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
    /// * `details` - A vector containing details of the country: [flag, code, name, currency, capital].
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let mut country_manager = Country::new();
    /// country_manager.insert_one("FR", vec!["🇫🇷", "33", "France", "EUR", "Paris"]);
    /// assert_eq!(country_manager.retrieve().len(), 9); // Check if the country was added
    /// ```
    pub fn insert_one(&mut self, short_name: &str, details: Vec<&str>) {
        let details_struct = Details {
            flag: details[0].to_string(),
            code: details[1].to_string(),
            name: details[2].to_string(),
            currency: details[3].to_string(),
            capital: details[4].to_string(),
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
    ///     ("IT", vec!["🇮🇹", "39", "Italy", "EUR", "Rome"]),
    ///     ("DE", vec!["🇩🇪", "49", "Germany", "EUR", "Berlin"]),
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

// Retrieves individual country details based on a specific key.
///
/// # Parameters
/// - country_code: A string slice containing the 2-letter country code (e.g., "US", "FR").
/// - key: A string slice specifying which detail to retrieve.
///          Possible values are "flag", "code", "name", and "currency", "capital".
///
/// # Returns
/// Returns a String representation of the requested detail. If the country_code
/// does not exist or if an invalid key is provided, an empty string is returned.
///
/// # Example
///
/// let flag = country_detail("US", "flag");  // Returns the flag emoji for the United States
/// let name = country_detail("US", "name");  // Returns "United States"
/// let invalid = country_detail("US", "invalid_key");  // Returns an empty string
///
pub fn country_detail(country_code: &str, key: &str) -> String {
    let ct = Country::new();

    // Create a binding for the details
    let details = ct.retrieve().get(country_code);

    match details {
        Some(detail) => match key {
            "flag" => detail.flag.clone(),         // Return flag
            "code" => detail.code.clone(),         // Return code
            "name" => detail.name.clone(),         // Return name
            "currency" => detail.currency.clone(), // Return currency
            "capital" => detail.capital.clone(),   // Return capital
            _ => String::new(),                    // Return an empty string for an invalid key
        },
        None => String::new(), // Return an empty string if the country code is not found
    }
}

/// Retrieves country details as a JSON object in string format.
///
/// # Parameters
/// - country_code: A string slice containing the 2-letter country code (e.g., "US", "FR").
///
/// # Returns
/// Returns a String that is a JSON representation of the country's details. If the
/// country_code does not exist, the returned string will contain empty values for
/// all fields: { "flag": "", "code": "", "name": "", "currency": "", "capital": "" }.
///
/// # Example
///
/// let details = country_details("US");  // Returns a JSON string with details of the United States
/// let invalid_details = country_details("XYZ");  // Returns a JSON string with empty details
///
pub fn country_details(country_code: &str) -> String {
    let ct = Country::new();

    // Create a binding for the details
    if let Some(detail) = ct.retrieve().get(country_code) {
        // Return a JSON string with all the details
        format!(
            "{{ \"flag\": \"{}\", \"code\": \"{}\", \"name\": \"{}\", \"currency\": \"{}\", \"capital\": \"{}\" }}",
            detail.flag, detail.code, detail.name, detail.currency, detail.capital
        )
    } else {
        // Return a JSON string with empty strings for not found
        String::from("{ \"flag\": \"\", \"code\": \"\", \"name\": \"\", \"currency\": \"\", \"capital\": \"\" }")
    }
}
