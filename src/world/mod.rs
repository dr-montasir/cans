use std::collections::HashMap;

/// Structure to represent a city.
#[derive(Debug, Clone)]
pub struct City<'a> {
    pub name: &'a str,      // Name of the city
    pub gmt: Vec<&'a str>,  // List of GMT time zones for the city
    pub latitude: &'a str,  // Latitude of the city
    pub longitude: &'a str, // Longitude of the city
    pub altitude: &'a str,  // Altitude of the city
}

// Struct to hold country details including cities.
///
/// # Example
///
/// ```rust
/// use cans::world::{Country, City};
///
/// let mut country_manager = Country::new();
/// println!("Current Data: {:?}", country_manager.retrieve());
///
/// // Inserting a new country with cities
/// country_manager.insert_one(
///     "FR",
///     vec!["🇫🇷", "33", "France", "EUR", "Paris"],
///     vec![
///         City {
///             name: "Paris",
///             gmt: vec!["GMT+1", "GMT+2"],
///             latitude: "48.8566",
///             longitude: "2.3522",
///             altitude: "35",
///         },
///         City {
///             name: "Lyon",
///             gmt: vec!["GMT+1", "GMT+2"],
///             latitude: "45.763420",
///             longitude: "4.834277",
///             altitude: "105",
///         },
///     ],
/// );
/// println!("After Inserting FR: {:?}", country_manager.retrieve());
///
/// // Inserting multiple countries at once
/// let new_countries = vec![
///     (
///         "IT",
///         vec!["🇮🇹", "39", "Italy", "EUR", "Rome"],
///         vec![
///             City {
///                 name: "Rome",
///                 gmt: vec!["GMT+1", "GMT+2"],
///                 latitude: "41.9028",
///                 longitude: "12.49637",
///                 altitude: "21",
///             },
///             City {
///                 name: "Milan",
///                 gmt: vec!["GMT+1", "GMT+2"],
///                 latitude: "45.464664",
///                 longitude: "9.188540",
///                 altitude: "122",
///             },
///         ],
///     ),
///     (
///         "DE",
///         vec!["🇩🇪", "49", "Germany", "EUR", "Berlin"],
///         vec![
///             City {
///                 name: "Berlin",
///                 gmt: vec!["GMT+1", "GMT+2"],
///                 latitude: "52.520008",
///                 longitude: "13.404954",
///                 altitude: "34",
///             },
///             City {
///                 name: "Munich",
///                 gmt: vec!["GMT+1", "GMT+2"],
///                 latitude: "48.137154",
///                 longitude: "11.576124",
///                 altitude: "520",
///             },
///         ],
///     ),
///     (
///         "FR",
///         vec!["🇫🇷", "33", "France", "EUR", "Paris"],
///         vec![
///             City {
///                 name: "Paris",
///                 gmt: vec!["GMT+1", "GMT+2"],
///                 latitude: "48.8566",
///                 longitude: "2.3522",
///                 altitude: "35",
///             },
///             City {
///                 name: "Lyon",
///                 gmt: vec!["GMT+1", "GMT+2"],
///                 latitude: "45.763420",
///                 longitude: "4.834277",
///                 altitude: "105",
///             },
///         ],
///     ),
/// ];
/// country_manager.insert_many(new_countries);
/// println!("After Inserting IT, DE and FR: {:?}", country_manager.retrieve());
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
pub struct Details<'a> {
    pub flag: &'a str,         // National flag emoji
    pub calling_code: &'a str, // Country calling code
    pub name: &'a str,         // Official country name
    pub currency: &'a str,     // Currency used in the country
    pub capital: &'a str,      // Capital city name
    pub cities: Vec<City<'a>>, // List of cities in the country
}

/// Struct to manage a collection of countries.
pub struct Country<'a> {
    data: HashMap<&'a str, Details<'a>>, // HashMap to store details of countries keyed by their short names
}

impl<'a> Country<'a> {
    // Creates a new Country instance with some initial example entries.
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

        // Entries including cities for each country
        let entries = [
            (
                "BH",
                "🇧🇭",
                "973",
                "Bahrain",
                "BHD",
                "Manama",
                vec![
                    City {
                        name: "Manama",
                        gmt: vec!["GMT+3"],
                        latitude: "26.22787",
                        longitude: "50.58565",
                        altitude: "10",
                    },
                    City {
                        name: "Riffa",
                        gmt: vec!["GMT+3"],
                        latitude: "26.129999",
                        longitude: "50.555000",
                        altitude: "15",
                    },
                ],
            ),
            (
                "CA",
                "🇨🇦",
                "1",
                "Canada",
                "CAD",
                "Ottawa",
                vec![
                    City {
                        name: "Ottawa",
                        gmt: vec!["GMT-5", "GMT-4"],
                        latitude: "45.4215",
                        longitude: "-75.6972",
                        altitude: "70",
                    },
                    City {
                        name: "Toronto",
                        gmt: vec!["GMT-5", "GMT-4"],
                        latitude: "43.65107",
                        longitude: "-79.347015",
                        altitude: "76.5",
                    },
                ],
            ),
            (
                "DZ",
                "🇩🇿",
                "213",
                "Algeria",
                "DZD",
                "Algiers",
                vec![
                    City {
                        name: "Algiers",
                        gmt: vec!["GMT+1"],
                        latitude: "36.737232",
                        longitude: "3.086472",
                        altitude: "424",
                    },
                    City {
                        name: "Oran",
                        gmt: vec!["GMT+1"],
                        latitude: "35.69694440",
                        longitude: "0.63305560",
                        altitude: "0.9",
                    },
                ],
            ),
            (
                "SA",
                "🇸🇦",
                "966",
                "Saudi Arabia",
                "SAR",
                "Riyadh",
                vec![
                    City {
                        name: "Riyadh",
                        gmt: vec!["GMT+3"],
                        latitude: "24.7136",
                        longitude: "46.6753",
                        altitude: "612",
                    },
                    City {
                        name: "Jeddah",
                        gmt: vec!["GMT+3"],
                        latitude: "21.2854",
                        longitude: "39.2376",
                        altitude: "12",
                    },
                ],
            ),
            (
                "SD",
                "🇸🇩",
                "249",
                "Sudan",
                "SDG",
                "Khartoum",
                vec![
                    City {
                        name: "Khartoum",
                        gmt: vec!["GMT+2"],
                        latitude: "15.5007",
                        longitude: "32.5599",
                        altitude: "385",
                    },
                    City {
                        name: "Omdurman",
                        gmt: vec!["GMT+2"],
                        latitude: "15.6866",
                        longitude: "32.4752",
                        altitude: "375",
                    },
                    City {
                        name: "Khartoum Bahri",
                        gmt: vec!["GMT+2"],
                        latitude: "15.6151",
                        longitude: "32.552",
                        altitude: "360",
                    },
                ],
            ),
            (
                "GB",
                "🇬🇧",
                "44",
                "United Kingdom",
                "GBP",
                "London",
                vec![
                    City {
                        name: "London",
                        gmt: vec!["GMT+0", "GMT+1"],
                        latitude: "51.509865",
                        longitude: "-0.118092",
                        altitude: "24",
                    },
                    City {
                        name: "Manchester",
                        gmt: vec!["GMT+0", "GMT+1"],
                        latitude: "53.4808",
                        longitude: "-2.2426",
                        altitude: "38",
                    },
                ],
            ),
            (
                "UK",
                "🇬🇧",
                "44",
                "United Kingdom",
                "GBP",
                "London",
                vec![
                    City {
                        name: "London",
                        gmt: vec!["GMT+0", "GMT+1"],
                        latitude: "51.509865",
                        longitude: "-0.118092",
                        altitude: "24",
                    },
                    City {
                        name: "Birmingham",
                        gmt: vec!["GMT+0", "GMT+1"],
                        latitude: "52.4862",
                        longitude: "-1.8904",
                        altitude: "150",
                    },
                ],
            ),
            (
                "US",
                "🇺🇸",
                "1",
                "United States",
                "USD",
                "Washington, D.C.",
                vec![
                    City {
                        name: "Washington, D.C.",
                        gmt: vec!["GMT-5", "GMT-4"],
                        latitude: "38.89511",
                        longitude: "-77.03637",
                        altitude: "125",
                    },
                    City {
                        name: "New York",
                        gmt: vec!["GMT-5", "GMT-4"],
                        latitude: "40.730610",
                        longitude: "-73.935242",
                        altitude: "10",
                    },
                ],
            ),
        ];

        for &(short_name, flag, calling_code, country_name, currency, capital, ref cities) in
            &entries
        {
            data.insert(
                short_name,
                Details {
                    flag,
                    calling_code,
                    name: country_name,
                    currency,
                    capital,
                    cities: cities.clone(),
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
    /// use cans::world::{Country, City};
    ///
    /// let mut country_manager = Country::new();
    /// println!("Current Data: {:?}", country_manager.retrieve());
    ///
    /// // Inserting a new country with cities
    /// country_manager.insert_one(
    ///     "FR",
    ///     vec!["🇫🇷", "33", "France", "EUR", "Paris"],
    ///     vec![
    ///         City {
    ///             name: "Paris",
    ///             gmt: vec!["GMT+1", "GMT+2"],
    ///             latitude: "48.8566",
    ///             longitude: "2.3522",
    ///             altitude: "35",
    ///         },
    ///         City {
    ///             name: "Lyon",
    ///             gmt: vec!["GMT+1", "GMT+2"],
    ///             latitude: "45.763420",
    ///             longitude: "4.834277",
    ///             altitude: "105",
    ///         },
    ///     ],
    /// );
    /// println!("After Inserting FR: {:?}", country_manager.retrieve());
    /// ```
    pub fn insert_one(
        &mut self,
        short_name: &'a str,
        details: Vec<&'a str>,
        cities: Vec<City<'a>>,
    ) {
        let details_struct = Details {
            flag: details[0],
            calling_code: details[1],
            name: details[2],
            currency: details[3],
            capital: details[4],
            cities,
        };
        self.data.insert(short_name, details_struct);
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
    /// use cans::world::{Country, City};
    ///
    /// let mut country_manager = Country::new();
    ///
    /// // Inserting multiple countries at once
    /// let new_countries = vec![
    ///     (
    ///         "IT",
    ///         vec!["🇮🇹", "39", "Italy", "EUR", "Rome"],
    ///         vec![
    ///         City {
    ///             name: "Rome",
    ///             gmt: vec!["GMT+1", "GMT+2"],
    ///             latitude: "41.9028",
    ///             longitude: "12.49637",
    ///             altitude: "21",
    ///         },
    ///         City {
    ///             name: "Milan",
    ///             gmt: vec!["GMT+1", "GMT+2"],
    ///             latitude: "45.464664",
    ///             longitude: "9.188540",
    ///             altitude: "122",
    ///         },
    ///         ],
    ///     ),
    ///     (
    ///         "DE",
    ///         vec!["🇩🇪", "49", "Germany", "EUR", "Berlin"],
    ///         vec![
    ///             City {
    ///                 name: "Berlin",
    ///                 gmt: vec!["GMT+1", "GMT+2"],
    ///                 latitude: "52.520008",
    ///                 longitude: "13.404954",
    ///                 altitude: "34",
    ///             },
    ///             City {
    ///                 name: "Munich",
    ///                 gmt: vec!["GMT+1", "GMT+2"],
    ///                 latitude: "48.137154",
    ///                 longitude: "11.576124",
    ///                 altitude: "520",
    ///             },
    ///         ],
    ///     ),
    ///     (
    ///         "FR",
    ///         vec!["🇫🇷", "33", "France", "EUR", "Paris"],
    ///         vec![
    ///             City {
    ///                 name: "Paris",
    ///                 gmt: vec!["GMT+1", "GMT+2"],
    ///                 latitude: "48.8566",
    ///                 longitude: "2.3522",
    ///                 altitude: "35",
    ///             },
    ///             City {
    ///                 name: "Lyon",
    ///                 gmt: vec!["GMT+1", "GMT+2"],
    ///                 latitude: "45.763420",
    ///                 longitude: "4.834277",
    ///                 altitude: "105",
    ///             },
    ///         ],
    ///     ),
    /// ];
    ///
    /// country_manager.insert_many(new_countries);
    /// println!("After Inserting IT, DE and FR: {:?}", country_manager.retrieve());
    /// ```
    pub fn insert_many(&mut self, entries: Vec<(&'a str, Vec<&'a str>, Vec<City<'a>>)>) {
        for (short_name, details, cities) in entries {
            self.insert_one(short_name, details, cities);
        }
    }

    /// Inserts a single city into an existing country.
    ///
    /// # Arguments
    ///
    /// * `country_code` - A string slice that holds the country code where the city will be added.
    /// * `new_city` - A `City` instance representing the new city to be added to the country.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cans::world::{Country, City};
    ///
    /// let mut country_manager = Country::new();
    /// let new_city = City {
    ///     name: "New City",
    ///     gmt: vec!["GMT-04:00"],
    ///     latitude: "41.147594",
    ///     longitude: "-73.989304",
    ///     altitude: "48",
    /// };
    /// country_manager.insert_one_city("US", new_city);
    /// ```
    pub fn insert_one_city(&mut self, country_code: &'a str, new_city: City<'a>) {
        if let Some(details) = self.data.get_mut(country_code) {
            // Check if the city already exists
            let mut replaced = false; // Flag to check if we replace an existing city
            for city in &mut details.cities {
                if city.name == new_city.name {
                    // Check by city name
                    *city = new_city.clone(); // Replace city if found
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                // If city was not found, add it to the list.
                details.cities.push(new_city);
            }
        }
    }

    /// Inserts multiple cities into an existing country.
    /// ```rust
    /// use cans::world::{Country, City};
    ///
    /// let mut country_manager = Country::new();
    /// let new_cities = vec![
    ///     City {
    ///         name: "Dallas",
    ///         gmt: vec!["GMT-06:00"],
    ///         latitude: "32.77666",
    ///         longitude: "-96.79699",
    ///         altitude: "129",
    ///     },
    ///     City {
    ///         name: "Michigan",
    ///         gmt: vec!["GMT-05:00"],
    ///         latitude: "44.978718",
    ///         longitude: "-84.515887",
    ///         altitude: "603",
    ///     },
    /// ];
    /// country_manager.insert_many_cities("US", new_cities);
    /// ```
    pub fn insert_many_cities(&mut self, country_code: &'a str, new_cities: Vec<City<'a>>) {
        if let Some(details) = self.data.get_mut(country_code) {
            for new_city in new_cities {
                let mut replaced = false; // Flag to track if a city was replaced
                for city in &mut details.cities {
                    if city.name == new_city.name {
                        // Check by city name
                        *city = new_city.clone(); // Replace city if found
                        replaced = true;
                        break;
                    }
                }
                if !replaced {
                    // If city was not found, add it to the list.
                    details.cities.push(new_city);
                }
            }
        }
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
    pub fn retrieve(&self) -> &HashMap<&'a str, Details<'a>> {
        &self.data
    }

    /// Sorts the countries in ascending order by name.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing references to the sorted country data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    ///
    /// let mut country_manager = Country::new();
    /// let sorted_asc = country_manager.sort_asc();
    /// println!("Countries Sorted Ascending: {:?}", sorted_asc);
    /// ```
    pub fn sort_asc(&self) -> Vec<&Details<'a>> {
        let mut sorted: Vec<&Details<'a>> = self.data.values().collect();
        sorted.sort_by(|a, b| a.name.cmp(b.name));
        sorted
    }

    /// Sorts the countries in descending order by name.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing references to the sorted country data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    ///
    /// let mut country_manager = Country::new();
    /// let sorted_desc = country_manager.sort_desc();
    /// println!("Countries Sorted Descending: {:?}", sorted_desc);
    /// ```
    pub fn sort_desc(&self) -> Vec<&Details<'a>> {
        let mut sorted: Vec<&Details<'a>> = self.data.values().collect();
        sorted.sort_by(|a, b| b.name.cmp(a.name)); // Reverse order for descending
        sorted
    }

    /// Retrieves individual country details based on a specific key.
    ///
    /// # Parameters
    /// - country_code: A string slice containing the 2-letter country code (e.g., "US", "FR").
    /// - key: A string slice specifying which detail to retrieve.
    ///          Possible values are "flag", "code", "name", and "currency", "capital", "cities".
    ///
    /// # Returns
    /// Returns a String representation of the requested detail. If the country_code
    /// does not exist or if an invalid key is provided, an empty string is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cans::world::Country;
    ///
    /// let mut country_manager = Country::new();
    ///
    /// let flag = country_manager.country_detail("US", "flag");  // Returns the flag emoji for the United States
    /// let name = country_manager.country_detail("US", "name");  // Returns "United States"
    /// let invalid = country_manager.country_detail("US", "invalid_key");  // Returns an empty string
    /// ```
    pub fn country_detail(&self, country_code: &'a str, key: &'a str) -> String {
        if let Some(detail) = self.retrieve().get(country_code) {
            match key {
                "flag" => detail.flag.to_string(),
                "calling_code" => detail.calling_code.to_string(),
                "name" => detail.name.to_string(),
                "currency" => detail.currency.to_string(),
                "capital" => detail.capital.to_string(),
                "cities" => {
                    let cities_json: Vec<String> = detail.cities.iter()
                      .map(|city| {
                          format!(
                              "{{ \"name\": \"{}\", \"gmt\": {:?}, \"latitude\": \"{}\", \"longitude\": \"{}\", \"altitude\": \"{}\" }}",
                              city.name, city.gmt, city.latitude, city.longitude, city.altitude
                          )
                      })
                      .collect();
                    format!("[{}]", cities_json.join(", ")) // Return as a JSON-like array
                }
                _ => String::new(), // Return empty string for an invalid key
            }
        } else {
            String::new() // Return empty string if the country short name code is not found
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
    /// all fields: { "flag": "", "code": "", "name": "", "currency": "", "capital": "", "cities": [] }.
    ///
    /// # Example
    ///
    /// let details = country_details("US");  // Returns a JSON string with details of the United States
    /// let invalid_details = country_details("XYZ");  // Returns a JSON string with empty details
    ///
    pub fn country_details(&self, country_code: &'a str) -> String {
        if let Some(detail) = self.retrieve().get(country_code) {
            let cities_json: Vec<String> = detail.cities.iter()
              .map(|city| {
                  format!(
                      "{{ \"name\": \"{}\", \"gmt\": {:?}, \"latitude\": \"{}\", \"longitude\": \"{}\", \"altitude\": \"{}\" }}",
                      city.name, city.gmt, city.latitude, city.longitude, city.altitude
                  )
              })
              .collect();

            // Construct JSON string with cities
            format!(
              "{{ \"flag\": \"{}\", \"calling_code\": \"{}\", \"name\": \"{}\", \"capital\": \"{}\", \"currency\": \"{}\", \"cities\": [{}] }}",
              detail.flag, detail.calling_code, detail.name, detail.capital, detail.currency, cities_json.join(", ")
          )
        } else {
            // Return empty JSON structure if country not found
            String::from("{ \"flag\": \"\", \"calling_code\": \"\", \"name\": \"\", \"capital\": \"\", \"currency\": \"\", \"cities\": [] }")
        }
    }

    /// Retrieves details of a specific city within a country.
    /// Retrieves details of a specific city within a country.
    ///
    /// # Parameters
    ///
    /// * `country_code` - A string slice that holds the 2-letter country code (e.g., "US", "FR") of the country
    ///   that contains the city.
    /// * `city_name` - A string slice that holds the name of the city for which details are being retrieved.
    /// * `key` - A string slice that specifies which detail to retrieve about the city. Possible values are:
    ///   - `"gmt"`: Retrieve the time zones the city is in.
    ///   - `"latitude"`: Retrieve the latitude of the city.
    ///   - `"longitude"`: Retrieve the longitude of the city.
    ///   - `"altitude"`: Retrieve the altitude of the city.
    ///   - `"name"`: Retrieve the name of the city itself.
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the requested detail about the city. If the city or country is not found,
    /// an empty string is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cans::world::{Country, City};
    ///
    /// let country_manager = Country::new();
    ///
    /// let gmt = country_manager.city_detail("US", "Washington, D.C.", "gmt");
    /// let latitude = country_manager.city_detail("US", "New York", "latitude");
    /// let invalid_key = country_manager.city_detail("US", "Washington, D.C.", "invalid_key");
    /// let non_existing_city = country_manager.city_detail("FR", "Nonexistent City", "name");
    ///
    /// assert_eq!(gmt, "GMT-5, GMT-4");  // Expected time zones for Washington, D.C.
    /// assert_eq!(latitude, "40.730610");  // Latitude for New York.
    /// assert!(invalid_key.is_empty());    // Should return an empty string for invalid key.
    /// assert!(non_existing_city.is_empty()); // Should return an empty string for a non-existing city.
    /// ```
    pub fn city_detail(&self, country_code: &'a str, city_name: &'a str, key: &'a str) -> String {
        if let Some(detail) = self.retrieve().get(country_code) {
            if let Some(city) = detail.cities.iter().find(|&c| c.name == city_name) {
                match key {
                    "gmt" => city.gmt.join(", "),
                    "latitude" => city.latitude.to_string(),
                    "longitude" => city.longitude.to_string(),
                    "altitude" => city.altitude.to_string(),
                    "name" => city.name.to_string(),
                    _ => String::new(),
                }
            } else {
                String::new() // Return empty string if city not found
            }
        } else {
            String::new() // Return empty string if country short name code is not found
        }
    }

    /// Retrieves details of a specific city as a JSON object.
    ///
    /// # Parameters
    ///
    /// * `country_code` - A string slice that holds the 2-letter country code (e.g., "US", "FR") of the country
    ///   that contains the city.
    /// * `city_name` - A string slice that holds the name of the city for which details are being retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `String` that represents a JSON object containing the city's details. If the city or country is
    /// not found, it will return a JSON string with empty fields:
    ///
    /// ```json
    /// { "name": "", "gmt": [], "latitude": "", "longitude": "", "altitude": "" }
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cans::world::{Country, City};
    ///
    /// let country_manager = Country::new();
    ///
    /// let city_json = country_manager.city_details("US", "Washington, D.C.");
    /// let non_existing_city_json = country_manager.city_details("FR", "Nonexistent City");
    ///
    /// // Expected JSON output for Washington, D.C.
    /// assert_eq!(city_json, r#"{ "name": "Washington, D.C.", "gmt": ["GMT-5", "GMT-4"], "latitude": "38.89511", "longitude": "-77.03637", "altitude": "125" }"#);
    ///
    /// // Expected JSON output for a non-existing city
    /// assert_eq!(non_existing_city_json, r#"{ "name": "", "gmt": [], "latitude": "", "longitude": "", "altitude": "" }"#);
    /// ```
    pub fn city_details(&self, country_code: &'a str, city_name: &'a str) -> String {
        if let Some(detail) = self.retrieve().get(country_code) {
            if let Some(city) = detail.cities.iter().find(|&c| c.name == city_name) {
                return format!(
                    "{{ \"name\": \"{}\", \"gmt\": {:?}, \"latitude\": \"{}\", \"longitude\": \"{}\", \"altitude\": \"{}\" }}",
                    city.name, city.gmt, city.latitude, city.longitude, city.altitude
                );
            }
        }
        String::from("{ \"name\": \"\", \"gmt\": [], \"latitude\": \"\", \"longitude\": \"\", \"altitude\": \"\" }")
        // Return empty if not found
    }

    /// Deletes a single city from an existing country.
    ///
    /// # Arguments
    ///
    /// * `country_code` - A string slice that holds the country code where the city will be deleted.
    /// * `city_name` - A string slice that contains the name of the city to be deleted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cans::world::{Country, City};
    ///
    /// let mut country_manager = Country::new();
    /// country_manager.delete_one_city("BH", "Manama");
    /// ```
    pub fn delete_one_city(&mut self, country_code: &'a str, city_name: &'a str) {
        if let Some(details) = self.data.get_mut(country_code) {
            details.cities.retain(|city| city.name != city_name); // Retain all cities that do not match the city_name
        }
    }

    /// Deletes multiple cities from an existing country.
    ///
    /// # Arguments
    ///
    /// * `country_code` - A string slice that holds the country code from which the cities will be deleted.
    /// * `city_names` - A slice of string slices containing the names of cities to be deleted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cans::world::{Country, City};
    ///
    /// let mut country_manager = Country::new();
    /// country_manager.delete_many_cities("CA", &["Ottawa", "Toronto"]);
    /// ```
    pub fn delete_many_cities(&mut self, country_code: &'a str, city_names: &[&'a str]) {
        if let Some(details) = self.data.get_mut(country_code) {
            details
                .cities
                .retain(|city| !city_names.contains(&city.name)); // Retain all cities that are not in city_names
        }
    }

    /// Deletes all cities from an existing country.
    ///
    /// # Arguments
    ///
    /// * `country_code` - A string slice that holds the country code from which all cities will be deleted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cans::world::{Country, City};
    ///
    /// let mut country_manager = Country::new();
    /// country_manager.delete_all_cities("UK");
    /// ```
    pub fn delete_all_cities(&mut self, country_code: &'a str) {
        if let Some(details) = self.data.get_mut(country_code) {
            details.cities.clear(); // Clear the cities list
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

    /// Deletes multiple countries from the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use cans::world::Country;
    /// let mut country_manager = Country::new();
    /// country_manager.delete_many(&["BH", "CA"]);
    /// assert_eq!(country_manager.retrieve().len(), 6); // Check if ["BH", "CA"] were deleted
    /// ```
    pub fn delete_many(&mut self, short_names: &[&str]) {
        for &short_name in short_names {
            self.data.remove(short_name);
        }
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
}
