
/// Determines the order of search results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/source_releases.html#order_by](https://research.stlouisfed.org/docs/api/fred/source_releases.html#order_by)
pub enum OrderBy {
    /// Default
    ReleaseId,
    Name,
    PressRelease,
    RealtimeStart,
    RealtimeEnd,
}

/// Sort order options for the fred/series/observation endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_search_tags.html#sort_order](https://research.stlouisfed.org/docs/api/fred/series_search_tags.html#sort_order)
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

    /// Initializes a new sources::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not check for duplicate arguments and instead adds all arguments to the URL string.  The FRED API behavior for duplicates in unknown.
    /// 
    /// ```
    /// use fred_rs::sources::Builder;
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
    /// [https://research.stlouisfed.org/docs/api/fred/source_releases.html#offset](https://research.stlouisfed.org/docs/api/fred/source_releases.html#offset)
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
            OrderBy::ReleaseId => {
                self.option_string += "&order_by=release_id";
            },
            OrderBy::Name => {
                self.option_string += "&order_by=name";
            },
            OrderBy::PressRelease => {
                self.option_string += "&order_by=press_release";
            },
            OrderBy::RealtimeStart => {
                self.option_string += "&order_by=realtime_start";
            },
            OrderBy::RealtimeEnd => {
                self.option_string += "&order_by=realtime_end";
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
    use crate::release::Response;
    use crate::client::FredClient;

    #[test]
    fn source_releases_with_options() {
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
            .order_by(OrderBy::Name)
            .sort_order(SortOrder::Descending);

        let resp: Response = match c.source_releases(1, Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.releases {
            match item.link {
                Some(l) => println!("{}: {}", item.name, l),
                None => println!("{}: No Link", item.name),
            }
        }
    } 
}