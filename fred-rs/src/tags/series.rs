use serde::Deserialize;

const TAG_NAME_REQUIRED_ERROR_TEXT: &str = "At least one tag must be specified using the tag_name() function of the related_tags::Builder.";

#[derive(Deserialize)]
/// Response data structure for the fred/tages/series endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags_series.html] (https://research.stlouisfed.org/docs/api/fred/tags_series.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// How the results are ordered
    pub order_by: String,
    // Results can be ascending (asc) or descending (desc)
    pub sort_order: String,
    /// Number of results returned
    pub count: usize,
    /// ???
    pub offset: usize,
    /// Maximum number of results to return
    pub limit: usize,
    /// Series returned by the search
    pub seriess: Vec<Series>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular data series
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags_series.html](https://research.stlouisfed.org/docs/api/fred/tags_series.html)
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
    /// Popularity score within the series group
    pub group_popularity: isize,
    /// Additional Notes
    pub notes: Option<String>,
}

/// Determines the type of search to perform
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags_series.html#search_type](https://research.stlouisfed.org/docs/api/fred/tags_series.html#search_type)
pub enum SearchType {
    /// (Default) Search series attributes including title, units, frequency and tags
    FullText,
    /// Search only the series ID number
    /// Wildcards are accepted with this options
    SeriesId,
}

/// Determines the order of search results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags_series.html#order_by](https://research.stlouisfed.org/docs/api/fred/tags_series.html#order_by)
pub enum OrderBy {
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

/// Sort order options for the fred/series/observation endpoint
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
    tag_names: String,
    exclude_tags: String,
}

impl Builder {

    /// Initializes a new tags::series::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not check for duplicate arguments and instead adds all arguments to the URL string.  The FRED API behavior for duplicates is unknown.
    /// 
    /// ```
    /// use fred_rs::tags::series::Builder;
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
            tag_names: String::new(),
            exclude_tags: String::new(),
        }
    }

    /// Returns the current arguments as a URL formatted string
    pub fn options(mut self) -> Result<String, String> {
        if self.tag_names.len() > 0 {
            self.option_string += format!("&tag_names={}", self.tag_names).as_str()
        } else {
            return Err(String::from(TAG_NAME_REQUIRED_ERROR_TEXT));
        }
        if self.exclude_tags.len() > 0 {
            self.option_string += format!("&exclude_tag_names={}", self.exclude_tags).as_str()
        }
        Ok(self.option_string)
    }

    /// Adds a tag name that all series must match
    /// 
    /// Results must match all included tag names.
    /// 
    /// # Arguments
    /// * `tag` - tag name to add
    pub fn tag_name(&mut self, tag: &str) -> &mut Builder {
        if self.tag_names.len() != 0 {
            self.tag_names.push(';');
        } 
        self.tag_names += tag;
        self
    }

    /// Adds a tag name that all series must exclude
    /// 
    /// Results must match no excluded tag names.
    /// 
    /// # Arguments
    /// * `tag` - tag name to add
    pub fn exclude_tag(&mut self, tag: &str) -> &mut Builder {
        if self.exclude_tags.len() != 0 {
            self.exclude_tags.push(';');
        } 
        self.exclude_tags += tag;
        self
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

    /// Adds a limit argument to the builder
    /// 
    /// The limit argument specifies a maximum number of observations to return.
    /// 
    /// # Arguments
    /// * `num_results` - Maximum number of results to return
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
    /// The API docs are rather vague on this argument so feel free to open an issue on GitHub with more information if you have it so I can update the docs.
    /// 
    /// https://research.stlouisfed.org/docs/api/fred/series_search.html#offset
    /// 
    /// # Arguments
    /// * `ofs` - the offset amount
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
        self
    }

    /// Adds the search_type argument to the request
    /// 
    /// # Arguments
    /// * `order` - result ranking system
    pub fn order_by(&mut self, order: OrderBy) -> &mut Builder {
        match order {
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
    fn tags_series_with_options_passing() {
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
            .tag_name("usa")
            .limit(5)
            .sort_order(SortOrder::Descending)
            .order_by(OrderBy::Frequency);

        let resp: Response = match c.tags_series(builder) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(1, 1);
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

    #[test]
    fn tags_series_with_options_failing() {
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

        let resp: Response = match c.tags_series(builder) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(1, 1);
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

        assert_eq!(2, 1); // if the request succeeded then failure
    } 
}