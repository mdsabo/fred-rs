//! Get an economic data series
//! 
//! [https://research.stlouisfed.org/docs/api/fred/series.html](https://research.stlouisfed.org/docs/api/fred/series.html)
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::series::{Builder, Response};
//! 
//! let mut c = match FredClient::new() {
//!     Ok(c) => c,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         assert_eq!(2, 1);
//!         return
//!     },
//! };
//! 
//! let mut builder = Builder::new();
//! builder.realtime_start("2000-01-01");
//! 
//! let resp: Response = match c.series("UNRATE", Some(builder)) {
//!     Ok(resp) => resp,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         assert_eq!(2, 1);
//!         return
//!     },
//! };
//! 
//! for item in resp.seriess {
//!     println!(
//!         "{}: {} {} {}",
//!         item.id,
//!         item.title,
//!         item.realtime_start,
//!         item.realtime_end
//!     );
//! }
//! ```

pub mod categories;
pub mod observation;
pub mod release;
pub mod tags;
pub mod search;
pub mod updates;
pub mod vintagedates;

// ----------------------------------------------------------------------------
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Clone, Debug, Default)]
/// Response data structure for the fred/series endpoint
/// 
/// Order_by, sort_order, count, offset and limit are used by endpoints which return a list of series.  They can be ignored for the fred/series endpoint.
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series.html] (https://research.stlouisfed.org/docs/api/fred/series.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// How the results are ordered
    pub order_by: Option<String>,
    /// Results can be ascending (asc) or descending (desc)
    pub sort_order: Option<String>,
    /// Number of results returned
    pub count: Option<usize>,
    /// ???
    pub offset: Option<usize>,
    /// Maximum number of results to return
    pub limit: Option<usize>,
    /// Series matching the requested series_id
    /// 
    /// The fred/series endpoint will return a series for each time a series changed.  For example Real GNP has been calculated several different ways over time so this endpoint will return a different series for each time period becasue they all fit under the same symbol: GNPCA.
    pub seriess: Vec<Series>,
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in self.seriess.iter() {
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
/// Data structure containing infomation about a particular data series
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series.html](https://research.stlouisfed.org/docs/api/fred/series.html)
pub struct Series {
    /// The series ID name
    pub id: String,
    /// The Real Time start of the series
    pub realtime_start: String,
    /// The Real Time end of the series
    pub realtime_end: String,
    /// The series title
    pub title: String,
    /// The series start date
    pub observation_start: String,
    /// The series end date
    pub observation_end: String,
    /// The series natural frequency (See [series::observation::Frequency])
    pub frequency: String,
    /// Short form of the frequency
    pub frequency_short: String,
    /// The data series units (e.g. Billions of Chanined 2009 Dollars)
    pub units: String,
    // Short form of the units (e.g. Bil. of Chn. 2009 $)
    pub units_short: String,
    /// Seasonal Adjustment Information
    pub seasonal_adjustment: String,
    /// Short form of the Seasonal Adjustment Info
    pub seasonal_adjustment_short: String,
    /// Date on whih the series was last updated
    pub last_updated: String,
    /// Popularity score
    pub popularity: isize,
    /// Group popularity score
    pub group_popularity: Option<isize>,
    /// Additional Notes
    pub notes: Option<String>,
}

impl Display for Series {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Series {}: {}", self.id, self.title)
    }
}

pub struct Builder {
    option_string: String
}

impl Builder {

    /// Initializes a new series::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::series::Builder;
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
    /// [https://research.stlouisfed.org/docs/api/fred/series.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/series.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/series.html#realtime_end)
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
    fn series_with_options() {
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
            .realtime_start("2000-01-01");

        let resp: Response = match c.series("UNRATE", Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.seriess {
            println!("{}: {} {} {}", item.id, item.title, item.realtime_start, item.realtime_end);
        }
    } 
}