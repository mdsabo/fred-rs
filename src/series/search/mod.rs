
/// Get the tags for a series search.
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_search_tags.html](https://research.stlouisfed.org/docs/api/fred/series_search_tags.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::search::tags::{Builder, Response, OrderBy, SortOrder};
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
///     .order_by(OrderBy::Popularity);
/// 
/// let resp: Response = match c.series_search_tags("monetary service index", Some(builder)) {
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

/// Get the related tags for a series search
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_search_related_tags.html](https://research.stlouisfed.org/docs/api/fred/series_search_related_tags.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::search::related_tags::{Builder, Response, OrderBy, SortOrder};
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
///     .order_by(OrderBy::Popularity);
/// 
/// let resp: Response = match c.series_search_related_tags("monetary service index", builder) {
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
pub mod related_tags;

// ----------------------------------------------------------------------------

/// Determines the type of search to perform
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_search.html#search_type](https://research.stlouisfed.org/docs/api/fred/series_search.html#search_type)
pub enum SearchType {
    /// (Default) Search series attributes including title, units, frequency and tags
    FullText,
    /// Search only the series ID number
    /// Wildcards are accepted with this option
    SeriesId,
}

/// Determines the order of search results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_search.html#order_by](https://research.stlouisfed.org/docs/api/fred/series_search.html#order_by)
pub enum OrderBy {
    /// Default if search type is FULL_TEXT
    SearchRank,
    /// Default if search type is SERIES_ID
    SeriesId,
    Title,
    Units,
    Frequency,
    SeasonalAdjustment,
    RealtimeStart,
    RealtimeEnd,
    LastUpdated,
    ObservationStart,
    ObservationEnd,
    Popularity,
    GroupPopularity,
}

/// Sort order options for the fred/series/search endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_search.html#sort_order](https://research.stlouisfed.org/docs/api/fred/series_search.html#sort_order)
pub enum SortOrder {
    /// Dates returned in ascending order (default)
    Ascending,    
    /// Dates returned in descending order
    Descending,   
}

/// Apply result filter
/// 
/// This should be used in conjunction with the filter_value argument to filter results based on one (maybe more than one?) of the fields.
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_search.html#filter_variable](https://research.stlouisfed.org/docs/api/fred/series_search.html#filter_variable)
pub enum FilterVariable {
    Frequency,
    Units,
    SeasonalAdjustment,
}

pub struct Builder {
    option_string: String,
    include_tags: String,
    exclude_tags: String,
}

impl Builder {

    /// Initializes a new series::search::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::series::search::Builder;
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
            include_tags: String::new(),
            exclude_tags: String::new(),
        }
    }

    /// Returns the current arguments as a URL formatted string
    pub fn options(mut self) -> String {
        if self.include_tags.len() > 0 {
            self.option_string += format!("&tag_names={}", self.include_tags).as_str()
        }
        if self.exclude_tags.len() > 0 {
            self.option_string += format!("&exclude_tag_names={}", self.exclude_tags).as_str()
        }
        self.option_string
    }

    /// Adds the search_type argument to the request
    /// 
    /// # Arguments
    /// * `stype` - search type (See SearchType enum)
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#search_type](https://research.stlouisfed.org/docs/api/fred/series_search.html#search_type)
    pub fn search_type(&mut self, stype: SearchType) -> &mut Builder {
        match stype {
            SearchType::SeriesId => {
                self.option_string += "&search_type=series_id";
            },
            _ => (), // FULL_TEXT is default
        };
        self
    }

    /// Adds a realtime_start argument to the builder
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/series_search.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/series_search.html#realtime_end)
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
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#limit](https://research.stlouisfed.org/docs/api/fred/series_search.html#limit)
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
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#offset](https://research.stlouisfed.org/docs/api/fred/series_search.html#offset)
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
        self
    }

    /// Adds the search_type argument to the request
    /// 
    /// # Arguments
    /// * `order` - result ranking system
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#order_by](https://research.stlouisfed.org/docs/api/fred/series_search.html#order_by)
    pub fn order_by(&mut self, order: OrderBy) -> &mut Builder {
        match order {
            OrderBy::SearchRank => {
                self.option_string += "&order_by=search_rank";
            },
            OrderBy::SeriesId => {
                self.option_string += "&order_by=series_id";
            },
            OrderBy::Title => {
                self.option_string += "&order_by=title";
            },
            OrderBy::Units => {
                self.option_string += "&order_by=units";
            },
            OrderBy::Frequency => {
                self.option_string += "&order_by=frequency";
            },
            OrderBy::SeasonalAdjustment => {
                self.option_string += "&order_by=seasonal_adjustment";
            },
            OrderBy::RealtimeStart => {
                self.option_string += "&order_by=realtime_start";
            },
            OrderBy::RealtimeEnd => {
                self.option_string += "&order_by=realtime_end";
            },
            OrderBy::LastUpdated => {
                self.option_string += "&order_by=last_updated";
            },
            OrderBy::ObservationStart => {
                self.option_string += "&order_by=observation_start";
            },
            OrderBy::ObservationEnd => {
                self.option_string += "&order_by=observation_end";
            },
            OrderBy::Popularity => {
                self.option_string += "&order_by=popularity";
            },
            OrderBy::GroupPopularity => {
                self.option_string += "&order_by=group_popularity";
            },
        };
        self
    }

    /// Change the sort order of the data
    /// 
    /// # Arguments
    /// * `order` - Data sort order enum
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#sort_order](https://research.stlouisfed.org/docs/api/fred/series_search.html#sort_order)
    pub fn sort_order(&mut self, order: SortOrder) -> &mut Builder {
        match order {
            SortOrder::Descending => {
                self.option_string += format!("&sort_order=desc").as_str()
            },
            _ => () // ASC is the default so do nothing
        }
        self
    }

    /// Adds the filter_variable argument to the request
    /// 
    /// # Arguments
    /// * `var` - the varible by which to filter
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#filter_variable](https://research.stlouisfed.org/docs/api/fred/series_search.html#filter_variable)
    pub fn filter_variable(&mut self, var: FilterVariable) -> &mut Builder {
        match var {
            FilterVariable::Frequency => {
                self.option_string += "&filter_variable=frequency";
            },
            FilterVariable::Units => {
                self.option_string += "&filter_variable=units";
            },
            FilterVariable::SeasonalAdjustment => {
                self.option_string += "&filter_variable=seasonal_adjustment";
            },
        };
        self
    }

    /// Sets the filter value for the specified filter variable
    /// 
    /// Results will only include a subset of the original results that match this value for the filter_variable argument.
    /// 
    /// # Arguments
    /// * `val` - the filter value
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#filter_value](https://research.stlouisfed.org/docs/api/fred/series_search.html#filter_value)
    pub fn filter_value(&mut self, val: &str) -> &mut Builder {
        self.option_string += format!("&filter_value={}", val).as_str();
        self
    }

    /// Adds a tag name to include in the search
    /// 
    /// Results must match all included tag names.
    /// 
    /// # Arguments
    /// * `tag` - tag name to add
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#tag_names](https://research.stlouisfed.org/docs/api/fred/series_search.html#tag_names)
    pub fn tag_name(&mut self, tag: &str) -> &mut Builder {
        if self.include_tags.len() != 0 {
            self.include_tags.push(';');
        } 
        self.include_tags += tag;
        self
    }

    /// Adds a tag name to exclude in the search
    /// 
    /// Results must match no excluded tag names.
    /// 
    /// # Arguments
    /// * `tag` - tag name to add
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_search.html#exclude_tag_names](https://research.stlouisfed.org/docs/api/fred/series_search.html#exclude_tag_names)
    pub fn exclude_tag(&mut self, tag: &str) -> &mut Builder {
        if self.exclude_tags.len() != 0 {
            self.exclude_tags.push(';');
        } 
        self.exclude_tags += tag;
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::Response;
    use crate::client::FredClient;

    #[test]
    fn series_search_with_options() {
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
            .sort_order(SortOrder::Descending)
            .order_by(OrderBy::Frequency);

        let resp: Response = match c.series_search("monetary index", Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.seriess {
            println!(
                "{}: {} {}",
                item.id,
                item.title,
                item.frequency,
            );
        }
    } 
}