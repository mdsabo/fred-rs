use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/series/search/related_tags endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_updates.html] (https://research.stlouisfed.org/docs/api/fred/series_updates.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// What variable the requested wass filtered with
    pub filter_variable: String,
    /// Value that must match the filter field to include a result
    pub filter_value: String,
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
    pub seriess: Vec<Series>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular data series
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_updates.html](https://research.stlouisfed.org/docs/api/fred/series_updates.html)
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
}

/// Used to filter series included in the results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_updates.html#filter_value](https://research.stlouisfed.org/docs/api/fred/series_updates.html#filter_value)
pub enum FilterValue {
    /// Macroeconomic data series
    Macro,
    /// Regional data series
    Regional,
    /// (Default) All data series
    All,
}

pub struct Builder {
    option_string: String,
}

impl Builder {

    /// Initializes a new series::updates::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not check for duplicate arguments and instead adds all arguments to the URL string.  The FRED API behavior for duplicates in unknown.
    /// 
    /// ```
    /// use fred_rs::series::updates::Builder;
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
    /// [https://research.stlouisfed.org/docs/api/fred/series_updates.html#offset](https://research.stlouisfed.org/docs/api/fred/series_updates.html#offset)
    /// 
    /// # Arguments
    /// * `ofs` - the offset amount
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
        self
    }

    /// Adds the filter_value argument to the request
    /// 
    /// # Arguments
    /// * `value` - value with which to filter results
    pub fn filter_value(&mut self, value: FilterValue) -> &mut Builder {
        match value {
            FilterValue::Macro => {
                self.option_string += "&filter_value=macro";
            },
            FilterValue::Regional => {
                self.option_string += "&filter_value=regional";
            },
            _ => (), // All is default so do nothing
        };
        self
    }

    /// Limit results to a certian time range
    /// 
    /// Both a start and end time must be specified together as per the API docs.
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_updates.html#start_time](https://research.stlouisfed.org/docs/api/fred/series_updates.html#start_time)
    /// 
    /// # Arguments
    /// * `start_time` - Start time to limit results to (YYYYMMDDHhmm format)
    /// * `end_time` - End time to limit results to (YYYYMMDDHhmm format)
    pub fn time_range(&mut self, start_time: &str, end_time: &str) -> &mut Builder {
        self.option_string += format!(
            "&start_time={}&end_time={}",
            start_time,
            end_time
        ).as_str();
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn series_updates_with_options() {
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
            .limit(5);

        let resp: Response = match c.series_updates(Some(builder)) {
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
                item.title,
                item.id,
                item.popularity,
            );
        }
    } 
}