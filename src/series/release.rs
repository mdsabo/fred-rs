//! Get the release for an economic data series
//! 
//! [https://research.stlouisfed.org/docs/api/fred/series_release.html](https://research.stlouisfed.org/docs/api/fred/series_release.html)
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::series::categories::Builder;
//! use fred_rs::category::Response;
//! 
//! // Create the client object
//! let mut c = match FredClient::new() {
//!     Ok(c) => c,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         return
//!     },
//! };
//! 
//! // Create the argument builder
//! let mut builder = Builder::new();
//! 
//! // Set the arguments for the builder
//! builder
//!     .realtime_start("2000-01-01")
//!     .realtime_end("2020-01-01");
//! 
//! // Make the request and pass in the builder to apply the arguments
//! let resp: Response = match c.series_categories("EXJPUS", Some(builder)) {
//!     Ok(resp) => resp,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         return
//!     },
//! };
//! ```

pub struct Builder {
    option_string: String
}

impl Builder {

    /// Initializes a new series::release::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::series::release::Builder;
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
    pub(crate) fn build(self) -> String {
        self.option_string
    }

    /// Adds a realtime_start argument to the builder
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_release.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/series_release.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_release.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/series_release.html#realtime_end)
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::release::Response;
    use crate::client::FredClient;

    #[test]
    fn series_release_with_options() {
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

        let resp: Response = match c.series_release("UNRATE", Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.releases {
            println!("{}: {}", item.name, item.press_release);
        }
    } 
}