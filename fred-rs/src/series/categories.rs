use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/series/categories endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_categories.html] (https://research.stlouisfed.org/docs/api/fred/series_categories.html)
pub struct Response {
    /// Categories within the specified series_id
    pub categories: Vec<Category>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular category
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_categories.html](https://research.stlouisfed.org/docs/api/fred/series_categories.html)
pub struct Category {
    /// The category ID number
    pub id: usize,
    /// The category name
    pub name: String,
    /// The parent category ID number
    pub parent_id: usize,
}

pub struct Builder {
    option_string: String
}

impl Builder {

    /// Initializes a new series::categories::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not check for duplicate arguments and instead adds all arguments to the URL string.  The FRED API behavior for duplicates in unknown.
    /// 
    /// ```
    /// use fred_rs::series::categories::Builder;
    /// // Create a new builder
    /// let mut builder = Builder::new();
    /// // add arguments to the builder
    /// builder
    ///     .realtime_start("1900-01-01")
    ///     .realtime_end("2000-01-01");
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

    /// Adds a realtime_start argument to the builder
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn series_categories_with_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let mut builder = Builder::new();
        builder.realtime_start("2000-01-01");

        let resp: Response = match c.series_categories("UNRATE", Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.categories {
            println!("{}: {} | Parent: {}", item.name, item.id, item.parent_id);
        }
    } 
}