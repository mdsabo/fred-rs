//! Get the tags for an economic data series
//! 
//! [https://research.stlouisfed.org/docs/api/fred/series_tags.html](https://research.stlouisfed.org/docs/api/fred/series_tags.html)
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::series::tags::{Builder, SortOrder, OrderBy};
//! use fred_rs::tags::Response;
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
//! builder
//!     .sort_order(SortOrder::Descending)
//!     .order_by(OrderBy::Popularity);
//! 
//! let resp: Response = match c.series_tags("STLFSI", Some(builder)) {
//!     Ok(resp) => resp,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         assert_eq!(2, 1);
//!         return
//!     },
//! };
//! 
//! for item in resp.tags {
//!     println!(
//!         "{}: {}",
//!         item.name,
//!         item.popularity,
//!     );
//! }
//! ```

/// Determines the order of search results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_tags.html#order_by](https://research.stlouisfed.org/docs/api/fred/series_tags.html#order_by)
pub enum OrderBy {
    /// Default
    SeriesCount,
    Popularity,
    Created,
    Name,
    GroupId,
}

/// Sort order options for the fred/series/tags endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_tags.html#sort_order](https://research.stlouisfed.org/docs/api/fred/series_tags.html#sort_order)
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

    /// Initializes a new series::search::tags::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::series::search::tags::Builder;
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
    /// [https://research.stlouisfed.org/docs/api/fred/series_tags.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/series_tags.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_tags.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/series_tags.html#realtime_end)
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }

    /// Adds the search_type argument to the request
    /// 
    /// # Arguments
    /// * `order` - result ranking system
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_tags.html#order_by](https://research.stlouisfed.org/docs/api/fred/series_tags.html#order_by)
    pub fn order_by(&mut self, order: OrderBy) -> &mut Builder {
        match order {
            OrderBy::SeriesCount => {
                self.option_string += "&order_by=series_count";
            },
            OrderBy::Popularity => {
                self.option_string += "&order_by=popularity";
            },
            OrderBy::Created => {
                self.option_string += "&order_by=created";
            },
            OrderBy::Name => {
                self.option_string += "&order_by=name";
            },
            OrderBy::GroupId => {
                self.option_string += "&order_by=group_id";
            },
        };
        self
    }

    /// Change the sort order of the data
    /// 
    /// # Arguments
    /// * `order` - Data sort order enum
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_tags.html#sort_order]9https://research.stlouisfed.org/docs/api/fred/series_tags.html#sort_order
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
    use crate::tags::Response;
    use crate::client::FredClient;

    #[test]
    fn series_tags_with_options() {
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
            .order_by(OrderBy::Popularity);

        let resp: Response = match c.series_tags("STLFSI", Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.tags {
            println!(
                "{}: {}",
                item.name,
                item.popularity,
            );
        }
    } 
}