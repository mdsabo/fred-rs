use serde::Deserialize;

#[derive(Deserialize)]
/// JSON object returned by the FRED API fred/series/observation endpoint
/// 
/// Individual datapoints (observations) are kept in a vector of observation objects.
/// 
/// The members primarily represent arguments passed to the API so for more specific info on each member see the associated ObservationBuilder argument function.
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_observations.html](https://research.stlouisfed.org/docs/api/fred/series_observations.html)
pub struct Response {
    pub realtime_start: String,
    pub realtime_end: String,
    pub observation_start: String,
    pub observation_end: String,
    pub units: String,
    pub output_type: usize,
    pub file_type: String,
    pub order_by: String,
    pub sort_order: String,
    pub count: usize,
    pub offset: usize,
    pub limit: usize,
    pub observations: Vec<DataPoint>,
}

#[derive(Deserialize)]
/// A single observation datapoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_observations.html](https://research.stlouisfed.org/docs/api/fred/series_observations.html)
pub struct DataPoint {
    pub realtime_start: String,
    pub realtime_end: String,
    /// Date of the data point
    pub date: String,
    /// String encoded data point
    pub value: String,
}

/// Argument builder for the fred/series/observation endpoint.
/// 
/// Each method adds an argument to the builder which can then be passed to the client used to fetch the data to apply the arguments.
pub struct Builder {
    option_string: String,
    vintage_dates: String,
}

/// Sort order options for the fred/series/observation endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_observations.html#sort_order](https://research.stlouisfed.org/docs/api/fred/series_observations.html#sort_order)
pub enum SortOrder {
    /// Dates returned in ascending order (default)
    Ascending,    
    /// Dates returned in descending order
    Descending,   
}

/// Data transformation options for the fred/series/observation endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_observations.html#units](https://research.stlouisfed.org/docs/api/fred/series_observations.html#units)
pub enum Units {
    /// Linear: no transform applied (default)
    LIN,
    /// Change: returns the period over period change of the observation
    CHG,
    /// 1 Year Change: Returns the YoY change of the observation
    CH1,
    /// Percent Change: Returns the period over period percent change of the observation
    PCH,
    /// 1 Year Percent Change: Returns the YoY percent change of the observation
    PC1,
    /// Compounded Annual Rate of Change
    PCA,
    /// Continuously Compounded Rate of Change
    CCH,
    /// Continuously Compounded Annual Rate of Change
    CCA,
    /// Natual Log: Returns the natural logarithm of the observation
    LOG,
}

/// Options for data series frequency
/// 
/// The frequency cannot exceed the native frequency of the data series.
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_observations.html#frequency](https://research.stlouisfed.org/docs/api/fred/series_observations.html#frequency)
pub enum Frequency {
    /// Daily (fastest)
    D,
    /// Weekly
    W,
    /// Bi-Weekly
    BW,
    /// Monthly
    M,
    /// Quarterly
    Q,
    /// Semi-Annualy
    SA,
    /// Annual (slowest)
    A,
    /// Weekly, Ending Friday
    WEF,    
    /// Weekly, Ending Thursday
    WETH,   
    /// Weekly, Ending Wednesday
    WEW,    
    /// Weekly, Ending Tuesday
    WETU,  
    /// Weekly, Ending Monday 
    WEM,   
    /// Weekly, Ending Sunday 
    WESU,  
    /// Weekly, Ending Saturday 
    WESA,  
    /// Bi-Weekly, Ending Wednesday 
    BWEW,   
    /// Bi-Weekly, Ending Monday
    BWEM,   
}

/// Provides an aggregation method for frequency aggregation
/// 
/// This argument should be used in conjunction with the frequency argument if the default aggregation method (AVG) is not preferred.
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_observations.html#aggregation_method](https://research.stlouisfed.org/docs/api/fred/series_observations.html#aggregation_method)
pub enum AggregationMethod {
    /// Average (default): intermediate datapoints are averaged to produce the aggregate
    AVG,
    /// Sum: intermediate datapoints are summed to produce the aggregate
    SUM,
    /// End of Period: The final result in the period is returned
    EOP
}

/// Specifies the data output type
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series_observations.html#output_type](https://research.stlouisfed.org/docs/api/fred/series_observations.html#output_type)
pub enum OutputType {
    /// Observations by Real Time Period
    RT,
    /// Observations by Vintage Date, All Observations
    VDALL,
    /// Observations by Vintage Date, New and Revised Observations Only
    VDNEW,
    /// Observations, Initial Relase Only
    INITIAL
}


impl Builder {
    /// Initializes a new observation::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not check for duplicate arguments and instead adds all arguments to the URL string.  The FRED API behavior for duplicates in unknown.
    /// 
    /// ```
    /// use fred_rs::series::observation::{Builder, Units, SortOrder};
    /// // Create a new builder
    /// let mut builder = Builder::new();
    /// // add arguments to the builder
    /// builder
    ///     .limit(100)
    ///     .units(Units::LOG)
    ///     .sort_order(SortOrder::Descending);
    /// ```
    pub fn new() -> Builder {
        Builder {
            option_string: String::new(),
            vintage_dates: String::new(),
        }
    }

    /// Returns the current arguments as a URL formatted string
    pub fn options(mut self) -> String {
        if self.vintage_dates.len() > 0 {
            self.option_string += format!("&vintage_dates={}", self.vintage_dates).as_str()
        }

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
    /// * `num_points` - Maximum number of data points to return
    pub fn limit(&mut self, num_points: usize) -> &mut Builder {
        let num_points = if num_points > 1000000 { // max value is 1000
            1000000
        } else {
            num_points
        };
        self.option_string += format!("&limit={}", num_points).as_str();
        self
    }

    /// Adds an offset argument to the builder
    /// 
    /// The API docs are rather vague on this argument so feel free to open an issue on GitHub with more information if you have it so I can update the docs.
    /// 
    /// https://research.stlouisfed.org/docs/api/fred/series_observations.html#offset
    /// 
    /// # Arguments
    /// * `ofs` - the offset amount
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
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
            _ => () // Ascending is the default so do nothing
        }
        self
    }

    /// Set the start year for data points
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    pub fn observation_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&observation_start={}", start_date).as_str();
        self
    }

    /// Set the end year for data points
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    pub fn observation_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&observation_end={}", end_date).as_str();
        self
    }

    /// Set the units of the data series
    /// 
    /// # Arguments
    /// * `units` - Data units to apply to the data set (see ObservationUnits)
    pub fn units(&mut self, units: Units) -> &mut Builder {
        match units {
            Units::CHG => {
                self.option_string += format!("&units=chg").as_str()
            },
            Units::CH1 => {
                self.option_string += format!("&units=ch1").as_str()
            },
            Units::PCH => {
                self.option_string += format!("&units=pch").as_str()
            },
            Units::PC1 => {
                self.option_string += format!("&units=pc1").as_str()
            },
            Units::PCA => {
                self.option_string += format!("&units=pca").as_str()
            },
            Units::CCH => {
                self.option_string += format!("&units=cch").as_str()
            },
            Units::CCA => {
                self.option_string += format!("&units=cca").as_str()
            },
            Units::LOG => {
                self.option_string += format!("&units=log").as_str()
            },
            _ => (), // lin is the default
        }
        self
    }

    /// Set the frequency of the data series
    /// 
    /// The requested frequency must be less than or equal to the native frequency for the data set.
    /// 
    /// # Arguments
    /// * `freq` - Frequency of data observations to return
    pub fn frequency(&mut self, freq: Frequency) -> &mut Builder {
        match freq {
            Frequency::D => {
                self.option_string += format!("&frequency=d").as_str()
            },
            Frequency::W => {
                self.option_string += format!("&frequency=w").as_str()
            },
            Frequency::BW => {
                self.option_string += format!("&frequency=bw").as_str()
            },
            Frequency::M => {
                self.option_string += format!("&frequency=m").as_str()
            },
            Frequency::Q => {
                self.option_string += format!("&frequency=q").as_str()
            },
            Frequency::SA => {
                self.option_string += format!("&frequency=sa").as_str()
            },
            Frequency::A => {
                self.option_string += format!("&frequency=a").as_str()
            },
            Frequency::WEF => {
                self.option_string += format!("&frequency=wef").as_str()
            },
            Frequency::WETH => {
                self.option_string += format!("&frequency=weth").as_str()
            },
            Frequency::WEW => {
                self.option_string += format!("&frequency=wew").as_str()
            },
            Frequency::WETU => {
                self.option_string += format!("&frequency=d").as_str()
            },
            Frequency::WEM => {
                self.option_string += format!("&frequency=wem").as_str()
            },
            Frequency::WESU => {
                self.option_string += format!("&frequency=wesu").as_str()
            },
            Frequency::WESA => {
                self.option_string += format!("&frequency=wesa").as_str()
            },
            Frequency::BWEW => {
                self.option_string += format!("&frequency=bwew").as_str()
            },
            Frequency::BWEM => {
                self.option_string += format!("&frequency=bwem").as_str()
            },
        }
        self
    }

    /// Set the aggregation method of the data series
    /// 
    /// # Arguments
    /// * `method` - See `ObservationAggregationMethod`
    pub fn aggregation_method(&mut self, method: AggregationMethod) -> &mut Builder {
        match method {
            AggregationMethod::SUM => {
                self.option_string += format!("&aggregation_method=sum").as_str()
            },
            AggregationMethod::EOP => {
                self.option_string += format!("&aggregation_method=eop").as_str()
            },
            _ => () // AVG is the default so do nothing
        }
        self
    }

    /// Set the datapoint output type
    /// 
    /// # Arguments
    /// * `otype` - See `ObservationOutputType`
    pub fn output_type(&mut self, otype: OutputType) -> &mut Builder {
        match otype {
            OutputType::VDALL => {
                self.option_string += format!("&output_type=2").as_str()
            },
            OutputType::VDNEW => {
                self.option_string += format!("&output_type=3").as_str()
            },
            OutputType::INITIAL => {
                self.option_string += format!("&output_type=4").as_str()
            },
            _ => () // AVG is the default so do nothing
        }
        self
    }

    /// Add a vintage date argument
    /// 
    /// This is the only parameter that could be added mroe than once.
    /// 
    /// The API accepts a comma separated list of vintage dates for which to return data.
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/series_observations.html#vintage_dates](https://research.stlouisfed.org/docs/api/fred/series_observations.html#vintage_dates)
    /// 
    /// # Arguments
    /// * `date` - date formatted as YYYY-MM-DD
    pub fn vintage_date(&mut self, date: &str) -> &mut Builder {
        if self.vintage_dates.len() != 0 {
            self.vintage_dates.push(',');
        } 
        self.vintage_dates += date;
        self
    }
}