
/// Get the categories for an economic data series
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_categories.html](https://research.stlouisfed.org/docs/api/fred/series_categories.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::categories::{Builder, Response};
/// 
/// // Create the client object
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         return
///     },
/// };
/// 
/// // Create the argument builder
/// let mut builder = Builder::new();
/// 
/// // Set the arguments for the builder
/// builder
///     .realtime_start("2000-01-01")
///     .realtime_end("2020-01-01");
/// 
/// // Make the request and pass in the builder to apply the arguments
/// let resp: Response = match c.series_categories("EXJPUS", Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         return
///     },
/// };
/// ```
pub mod categories;

/// Get the observations (data points) for an economic data series
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_observations.html](https://research.stlouisfed.org/docs/api/fred/series_observations.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::observation::{Builder, Units, Frequency, Response};
/// 
/// // Create the client object
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         return
///     },
/// };
/// 
/// // Create the argument builder
/// let mut builder = Builder::new();
/// 
/// // Set the arguments for the builder
/// builder
///     .observation_start("2000-01-01")
///     .units(Units::PCH)
///     .frequency(Frequency::M);
/// 
/// // Make the request and pass in the builder to apply the arguments
/// let resp: Response = match c.series_observation("GNPCA", Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         return
///     },
/// };
/// ```
pub mod observation;

/// Get the release for an economic data series
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_release.html](https://research.stlouisfed.org/docs/api/fred/series_release.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::categories::{Builder, Response};
/// 
/// // Create the client object
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         return
///     },
/// };
/// 
/// // Create the argument builder
/// let mut builder = Builder::new();
/// 
/// // Set the arguments for the builder
/// builder
///     .realtime_start("2000-01-01")
///     .realtime_end("2020-01-01");
/// 
/// // Make the request and pass in the builder to apply the arguments
/// let resp: Response = match c.series_categories("EXJPUS", Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         return
///     },
/// };
/// ```
pub mod release;

/// Get the tags for an economic data series
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::tags::{Builder, Response, SortOrder, OrderBy};
/// 
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let mut builder = Builder::new();
/// builder
///     .sort_order(SortOrder::Descending)
///     .order_by(OrderBy::Popularity);
/// 
/// let resp: Response = match c.series_tags("STLFSI", Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.tags {
///     println!(
///         "{}: {}",
///         item.name,
///         item.popularity,
///     );
/// }
/// ```

pub mod tags;

// ----------------------------------------------------------------------------

/// Get economic data series that match keywords
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::search::{Builder, Response, OrderBy, SortOrder};
/// 
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let mut builder = Builder::new();
/// builder
///     .limit(5)
///     .sort_order(SortOrder::Descending)
///     .order_by(OrderBy::Frequency);
/// 
/// let resp: Response = match c.series_search("monetary index", Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.seriess {
///     println!(
///         "{}: {} {}",
///         item.id,
///         item.title,
///         item.frequency,
///     );
/// }
/// ```
pub mod search;

// ----------------------------------------------------------------------------
use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/series endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series.html] (https://research.stlouisfed.org/docs/api/fred/series.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// Series matching the requested series_id
    /// 
    /// The fred/series endpoint will return a series for each time a series changed.  For example Real GNP has been calculated several different ways over time so this endpoint will return a different series for each time period becasue they all fit under the same symbol: GNPCA.
    pub seriess: Vec<Series>,
}

#[derive(Deserialize)]
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
    pub popularity: usize,
    /// Additional Notes
    pub notes: String,
}

pub struct Builder {
    option_string: String
}

impl Builder {

    /// Initializes a new series::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not check for duplicate arguments and instead adds all arguments to the URL string.  The FRED API behavior for duplicates in unknown.
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
        builder.realtime_start("2000-01-01");

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