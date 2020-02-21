//! Get the dates in history when a series' data values were revised or new data values were released
//! 
//! [https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html](https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html)
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::series::vintagedates::{Builder, Response, SortOrder};
//! 
//! let mut c = match FredClient::new() {
//! Ok(c) => c,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         assert_eq!(2, 1);
//!         return
//!     },
//! };
//! 
//! let mut builder = Builder::new();
//! builder
//!     .sort_order(SortOrder::Descending)
//!     .limit(5);
//! 
//! let resp: Response = match c.series_vintagedates("GNPCA", Some(builder)) {
//!     Ok(resp) => resp,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         assert_eq!(2, 1);
//!         return
//!     },
//! };
//! 
//! for item in resp.vintage_dates {
//!     println!("{}", item);
//! }
//! ```

use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/series/vintagedates endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html] (https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// How the results are ordered
    pub order_by: String,
    // Results are listed in ascending or descending
    pub sort_order: String,
    /// Number of results returned
    pub count: usize,
    /// ???
    pub offset: usize,
    /// Maximum number of results to return
    pub limit: usize,
    /// Series returned by the search
    pub vintage_dates: Vec<String>,
}

/// Sort order options for the fred/series/vintagedates endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#sort_order](https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#sort_order)
pub enum SortOrder {
    /// Dates returned in ascending order (default)
    Ascending,    
    /// Dates returned in descending order
    Descending,   
}

pub struct Builder {
    option_string: String,
}

impl Builder {

    /// Initializes a new series::vintagedates::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::series::vintagedates::Builder;
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
    /// 
    /// Returns Err if there are not tag names specified using tag_name().
    pub fn options(self) -> String {
        self.option_string
    }

    /// Adds a realtime_start argument to the builder
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#realtime_end)
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }

    /// Adds a limit argument to the builder
    /// 
    /// The limit argument specifies a maximum number of observations to return.
    /// 
    /// # Arguments
    /// * `num_results` - Maximum number of results to return
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#limit](https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#limit)
    pub fn limit(&mut self, num_results: usize) -> &mut Builder {
        let num_results = if num_results > 10000 { // max value is 1000
            10000
        } else {
            num_results
        };
        self.option_string += format!("&limit={}", num_results).as_str();
        self
    }

    /// Adds an offset argument to the builder
    /// 
    /// Adding an offset shifts the starting result number.  For example, if limit is 5 and offset is 0 then results 1-5 will be returned, but if offset was 5 then results 6-10 would be returned.
    /// 
    /// # Arguments
    /// * `ofs` - the offset amount
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#offset](https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#offset)
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
        self
    }

    /// Change the sort order of the data
    /// 
    /// # Arguments
    /// * `order` - Data sort order enum
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#sort_order](https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#sort_order)
    pub fn sort_order(&mut self, order: SortOrder) -> &mut Builder {
        match order {
            SortOrder::Descending => {
                self.option_string += format!("&sort_order=desc").as_str()
            },
            _ => () // ASC is the default so do nothing
        }
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn series_vintagedates_with_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let mut builder = Builder::new();
        builder
            .sort_order(SortOrder::Descending)
            .limit(5);

        let resp: Response = match c.series_vintagedates("GNPCA", Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.vintage_dates {
            println!("{}", item);
        }
    } 
}