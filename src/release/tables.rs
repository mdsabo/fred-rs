use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
/// Response data structure for the fred/release/tables endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_tables.html] (https://research.stlouisfed.org/docs/api/fred/release_tables.html)
pub struct Response {
    /// The name of the release
    pub name: Option<String>,
    /// The table element ID number
    pub element_id: Option<usize>,
    /// The release ID number that was queried
    pub release_id: String,
    /// List of releases related to the specified series_id
    pub elements: HashMap<String, Element>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular release table element
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_tables.html](https://research.stlouisfed.org/docs/api/fred/release_tables.html)
pub struct Element {
    /// The element ID number
    pub element_id: usize,
    /// The release ID number
    pub release_id: usize,
    /// The series name
    pub series_id: Option<String>,
    /// The parent element ID number
    pub parent_id: Option<usize>,
    /// The table line number
    pub line: Option<String>,
    /// The element type
    #[serde(rename = "type")]
    pub etype: String,
    /// The element name
    pub name: String,
    /// The element nesting level
    pub level: String,
    // Children of this element
    pub children: Vec<Element>,
}

pub struct Builder {
    option_string: String
}

impl Builder {

    /// Initializes a new release::tables::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::release::tables::Builder;
    /// // Create a new builder
    /// let mut builder = Builder::new();
    /// // add arguments to the builder
    /// builder
    ///     .include_observation_values();
    /// ```
    pub fn new() -> Builder {
        Builder {
            option_string: String::new(),
        }
    }

    /// Returns the current arguments as a URL formatted string
    pub fn options(self) -> String {
        self.option_string
    }

    /// Add the element_id argument to the builder
    /// 
    /// # Arguments
    /// * `id` - The release table element id you would like to retrieve
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_tables.html#element_id](https://research.stlouisfed.org/docs/api/fred/release_tables.html#element_id)
    pub fn element_id(&mut self, id: usize) -> &mut Builder {
        self.option_string += format!("&element_id={}", id).as_str();
        self
    }

    /// Use to indicate that observation values should be returned as well for applicable series
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_tables.html#include_observation_values](https://research.stlouisfed.org/docs/api/fred/release_tables.html#include_observation_values)
    pub fn include_observation_values(&mut self) -> &mut Builder {
        self.option_string += "&include_observation_values=true";
        self
    }

    /// Adds an observation date to be returned with the release table
    /// 
    /// # Arguments
    /// * `date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_tables.html#observation_date](https://research.stlouisfed.org/docs/api/fred/release_tables.html#observation_date)
    pub fn observation_date(&mut self, date: &str) -> &mut Builder {
        self.option_string += format!("&observation_date={}", date).as_str();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn release_tables_with_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let mut builder = Builder::new();
        builder.include_observation_values();

        let resp: Response = match c.release_tables(53, Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for (key, value) in resp.elements {
            println!("{}: {}", key, value.name);
        }
    } 
}