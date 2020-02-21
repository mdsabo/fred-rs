// MIT License
// 
// Copyright (c) 2020 Matthew Sabo
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Get the series on a release of economic data
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_series.html](https://research.stlouisfed.org/docs/api/fred/release_series.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::release::series::{Builder, OrderBy, SortOrder};
/// use fred_rs::series::Response;
/// 
/// let mut c = match FredClient::new() {
/// Ok(c) => c,
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
/// let resp: Response = match c.release_series(9, Some(builder)) {
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
pub mod series;

/// Get the sources for a release of economic data
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_sources.html](https://research.stlouisfed.org/docs/api/fred/release_sources.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::release::sources::Builder;
/// use fred_rs::source::Response;
/// 
/// let mut c = match FredClient::new() {
/// Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let mut builder = Builder::new();
/// builder
///     .realtime_start("2000-01-01");
/// 
/// let resp: Response = match c.release_sources(9, Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.sources {
///     println!("{}: {}", item.id, item.name);
/// }
/// ```
pub mod sources;

/// Get the tags for a release
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_tags.html](https://research.stlouisfed.org/docs/api/fred/release_tags.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::release::tags::{Builder, OrderBy, SortOrder};
/// use fred_rs::tags::Response;
/// 
/// let mut c = match FredClient::new() {
/// Ok(c) => c,
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
///     .order_by(OrderBy::Created);
/// 
/// let resp: Response = match c.release_tags(9, Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.tags {
///     println!("{}: {}", item.name, item.created);
/// }
/// ```
pub mod tags;

/// Get the related tags for a release
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::release::related_tags::{Builder, OrderBy, SortOrder};
/// use fred_rs::tags::Response;
/// 
/// let mut c = match FredClient::new() {
/// Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let mut builder = Builder::new();
/// builder
///     .tag_name("usa")
///     .limit(5)
///     .sort_order(SortOrder::Descending)
///     .order_by(OrderBy::Created);
/// 
/// let resp: Response = match c.release_related_tags(9, builder) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.tags {
///     println!("{}: {}", item.name, item.created);
/// }
/// ```
pub mod related_tags;

/// Get the release tables for a given release
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_tables.html](https://research.stlouisfed.org/docs/api/fred/release_tables.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::release::tables::{Builder, Response};
/// 
/// let mut c = match FredClient::new() {
/// Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let mut builder = Builder::new();
/// builder
///     .include_observation_values();
/// 
/// let resp: Response = match c.release_tables(9, Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for (key, value) in resp.elements {
///     println!("{}: {}", key, value.name);
/// }
/// ```
pub mod tables;

// -----------------------------------------------------------------------------
use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/release endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release.html] (https://research.stlouisfed.org/docs/api/fred/release.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// How the results are ordered
    pub order_by: Option<String>,
    // Results are listed in ascending or descending
    pub sort_order: Option<String>,
    /// Number of results returned
    pub count: Option<usize>,
    /// ???
    pub offset: Option<usize>,
    /// Maximum number of results to return
    pub limit: Option<usize>,
    /// List of releases related to the specified series_id
    pub releases: Vec<Release>,
}

#[derive(Deserialize)]
/// Data structure containing information about a particular release
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release.html](https://research.stlouisfed.org/docs/api/fred/release.html)
pub struct Release {
    /// The category ID number
    pub id: usize,
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// The releaase name
    pub name: String,
    /// Indicates if there was a press release
    pub press_release: bool,
    /// A link to the press release if there was one
    pub link: Option<String>,
    /// Addition notes about the release
    pub notes: Option<String>
}

pub struct Builder {
    option_string: String
}

impl Builder {

    /// Initializes a new series::release::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::release::Builder;
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
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/release.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/release.html#realtime_end)
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
    fn release_with_options() {
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

        let resp: Response = match c.release(9, Some(builder)) {
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