//! Get release dates for all releases of economic data
//! 
//! [https://research.stlouisfed.org/docs/api/fred/releases_dates.html](https://research.stlouisfed.org/docs/api/fred/releases_dates.html)
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::releases::dates::{Builder, Response, OrderBy, SortOrder};
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
//! builder
//!     .limit(5)
//!     .sort_order(SortOrder::Ascending)
//!     .order_by(OrderBy::ReleaseId);
//! 
//! // Make the request and pass in the builder to apply the arguments
//! let resp: Response = match c.releases_dates(Some(builder)) {
//!     Ok(resp) => resp,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         return
//!     },
//! };
//! ```

use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Clone, Debug, Default)]
/// Response data structure for the fred/releases/dates endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html] (https://research.stlouisfed.org/docs/api/fred/releases_dates.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// How the results are ordered
    pub order_by: String,
    // Results are listed in ascending or descending order
    pub sort_order: String,
    /// Number of results returned
    pub count: usize,
    /// First result returned
    pub offset: usize,
    /// Maximum number of results to return
    pub limit: usize,
    /// List of release dates for a release
    pub release_dates: Vec<ReleaseDate>,
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in self.release_dates.iter() {
            match item.fmt(f) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            match writeln!(f, "") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Clone, Debug, Default)]
/// Data structure containing infomation about a particular release
/// 
/// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html](https://research.stlouisfed.org/docs/api/fred/releases_dates.html)
pub struct ReleaseDate {
    /// The release ID number
    pub release_id: usize,
    /// The name of the release
    pub release_name: Option<String>,
    /// The date of the release
    pub date: String,
}

impl Display for ReleaseDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Release Date {}: {}", self.release_id, self.date)
    }
}

/// Determines the order of search results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#order_by](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#order_by)
pub enum OrderBy {
    /// Default
    ReleaseDate,
    ReleaseId,
    ReleaseName,
}

/// Sort order options for the fred/releases/dates endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#sort_order](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#sort_order)
pub enum SortOrder {
    /// Dates returned in ascending order
    Ascending,    
    /// Dates returned in descending order (default)
    Descending,   
}

pub struct Builder {
    option_string: String
}

impl Builder {

    /// Initializes a new releases::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::releases::dates::Builder;
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
    /// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#realtime_end)
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
    /// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#limit](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#limit)
    pub fn limit(&mut self, num_results: usize) -> &mut Builder {
        let num_results = if num_results > 1000 { // max value is 1000
            1000
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
    /// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#offset](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#offset)
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
        self
    }

    /// Adds the search_type argument to the request
    /// 
    /// # Arguments
    /// * `order` - result ranking system
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#order_by](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#order_by)
    pub fn order_by(&mut self, order: OrderBy) -> &mut Builder {
        match order {
            OrderBy::ReleaseDate => {
                self.option_string += "&order_by=release_name";
            },
            OrderBy::ReleaseId => {
                self.option_string += "&order_by=release_id";
            },
            OrderBy::ReleaseName => {
                self.option_string += "&order_by=name";
            },
        };
        self
    }

    /// Change the sort order of the data
    /// 
    /// # Arguments
    /// * `order` - Data sort order enum
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#sort_order](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#sort_order)
    pub fn sort_order(&mut self, order: SortOrder) -> &mut Builder {
        match order {
            SortOrder::Ascending => {
                self.option_string += format!("&sort_order=asc").as_str()
            },
            _ => () // DESC is the default so do nothing
        }
        self
    }

    /// Include release dates with no data available
    /// 
    /// The default is false.  Calling this will set the argument to true.
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases_dates.html#include_release_dates_with_no_data](https://research.stlouisfed.org/docs/api/fred/releases_dates.html#include_release_dates_with_no_data)
    pub fn include_release_dates_with_no_data(&mut self) -> &mut Builder {
        self.option_string += format!(
            "&include_release_dates_with_no_data=true"
        ).as_str();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn releases_dates_with_options() {
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
            .limit(5)
            .sort_order(SortOrder::Ascending)
            .order_by(OrderBy::ReleaseId);

        let resp: Response = match c.releases_dates(Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.release_dates {
            println!("{}: {} -> {}", item.date, item.release_id, item.release_name.unwrap());
        }
    } 
}