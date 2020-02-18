
use serde::Deserialize;

// DOCS: https://research.stlouisfed.org/docs/api/fred/series_observations.html
// BASE_URL/series/observations?
// Arguments:
// api_key
// series_id: ID for the series
// Options:
// file_type: xml (default), json, txt, xls
// realtime_start: effects reported values
// realtime_end: #s are reported in that time's values
// limit: max # of results
// offset: offset from now, unsigned int (i guess)
// sort_order: "asc" or "desc", default: "asc"
pub enum ObservationSortOrder {
    ASC,
    DESC,
}
// observation_start: start date as YYYY-MM-DD
// observation_end: end date as YYYY-MM-DD
// units: data transforms, see docs for more info
pub enum ObservationUnits {
    LIN, // linear: no transform
    CHG, // Change
    CH1, // 1Y Change
    PCH, // % Change
    PC1, // 1Y % Change
    PCA, // Compounded Annual Rate of Change
    CCH, // Continuously Compounded Rate of Change
    CCA, // Continuously Compounded Annual Rate of Change
    LOG, // Natural Log
}
// frequency: data frequency, see docs for options
pub enum ObservationFrequency {
    D,      // Daily
    W,      // Weekly
    BW,     // Bi-Weekly
    M,      // Monthly
    Q,      // Quarterly
    SA,     // Semi-Annualy
    A,      // Annual
    WEF,    // Weekly, Ending Friday
    WETH,   // Weekly, Ending Thursday
    WEW,    // Weekly, Ending Wednesday
    WETU,   // Weekly, Ending Tuesday
    WEM,    // Weekly, Ending Monday
    WESU,   // Weekly, Ending Sunday
    WESA,   // Weekly, Ending Saturday
    BWEW,   // Bi-Weekly, Ending Wednesday
    BWEM,   // Bi-Weekly, Ending Monday
}
// aggregation_method: frequency aggregation method - 'sum', 'avg' or 'eop'
pub enum ObservationAggregationMethod {
    AVG,
    SUM,
    EOP
}
// output_type: see docs
pub enum ObservationOutputType {
    RT,
    VDALL,
    VDNEW,
    INITIAL
}
// vintage_dates: string of dates, returns data as it existed on those dates
#[derive(Deserialize)]
pub struct ObservationResponse {
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
    pub observations: Vec<Observation>,
}

// A single observation value
#[derive(Deserialize)]
pub struct Observation {
    pub realtime_start: String,
    pub realtime_end: String,
    pub date: String,
    pub value: String,
}

pub struct ObservationBuilder {
    option_string: String,
    vintage_dates: String,
}

impl ObservationBuilder {
    pub fn new() -> ObservationBuilder {
        ObservationBuilder {
            option_string: String::new(),
            vintage_dates: String::new(),
        }
    }

    pub fn options(mut self) -> String {
        if self.vintage_dates.len() > 0 {
            self.option_string += format!("&vintage_dates={}", self.vintage_dates).as_str()
        }

        self.option_string
    }

    pub fn realtime_start(&mut self, start_date: &str) -> &mut ObservationBuilder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }
    pub fn realtime_end(&mut self, start_end: &str) -> &mut ObservationBuilder {
        self.option_string += format!("&realtime_end={}", start_end).as_str();
        self
    }
    pub fn limit(&mut self, limit: usize) -> &mut ObservationBuilder {
        self.option_string += format!("&limit={}", limit).as_str();
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut ObservationBuilder {
        self.option_string += format!("&offset={}", offset).as_str();
        self
    }
    pub fn sort_order(&mut self, order: ObservationSortOrder) -> &mut ObservationBuilder {
        match order {
            ObservationSortOrder::DESC => {
                self.option_string += format!("&sort_order=desc").as_str()
            },
            _ => () // ASC is the default so do nothing
        }
        self
    }
    pub fn observation_start(&mut self, start_date: &str) -> &mut ObservationBuilder {
        self.option_string += format!("&observation_start={}", start_date).as_str();
        self
    }
    pub fn observation_end(&mut self, start_end: &str) -> &mut ObservationBuilder {
        self.option_string += format!("&observation_end={}", start_end).as_str();
        self
    }
    pub fn units(&mut self, order: ObservationUnits) -> &mut ObservationBuilder {
        match order {
            ObservationUnits::CHG => {
                self.option_string += format!("&units=chg").as_str()
            },
            ObservationUnits::CH1 => {
                self.option_string += format!("&units=ch1").as_str()
            },
            ObservationUnits::PCH => {
                self.option_string += format!("&units=pch").as_str()
            },
            ObservationUnits::PC1 => {
                self.option_string += format!("&units=pc1").as_str()
            },
            ObservationUnits::PCA => {
                self.option_string += format!("&units=pca").as_str()
            },
            ObservationUnits::CCH => {
                self.option_string += format!("&units=cch").as_str()
            },
            ObservationUnits::CCA => {
                self.option_string += format!("&units=cca").as_str()
            },
            ObservationUnits::LOG => {
                self.option_string += format!("&units=log").as_str()
            },
            _ => (), // lin is the default
        }
        self
    }
    pub fn frequency(&mut self, order: ObservationFrequency) -> &mut ObservationBuilder {
        match order {
            ObservationFrequency::D => {
                self.option_string += format!("&frequency=d").as_str()
            },
            ObservationFrequency::W => {
                self.option_string += format!("&frequency=w").as_str()
            },
            ObservationFrequency::BW => {
                self.option_string += format!("&frequency=bw").as_str()
            },
            ObservationFrequency::M => {
                self.option_string += format!("&frequency=m").as_str()
            },
            ObservationFrequency::Q => {
                self.option_string += format!("&frequency=q").as_str()
            },
            ObservationFrequency::SA => {
                self.option_string += format!("&frequency=sa").as_str()
            },
            ObservationFrequency::A => {
                self.option_string += format!("&frequency=a").as_str()
            },
            ObservationFrequency::WEF => {
                self.option_string += format!("&frequency=wef").as_str()
            },
            ObservationFrequency::WETH => {
                self.option_string += format!("&frequency=weth").as_str()
            },
            ObservationFrequency::WEW => {
                self.option_string += format!("&frequency=wew").as_str()
            },
            ObservationFrequency::WETU => {
                self.option_string += format!("&frequency=d").as_str()
            },
            ObservationFrequency::WEM => {
                self.option_string += format!("&frequency=wem").as_str()
            },
            ObservationFrequency::WESU => {
                self.option_string += format!("&frequency=wesu").as_str()
            },
            ObservationFrequency::WESA => {
                self.option_string += format!("&frequency=wesa").as_str()
            },
            ObservationFrequency::BWEW => {
                self.option_string += format!("&frequency=bwew").as_str()
            },
            ObservationFrequency::BWEM => {
                self.option_string += format!("&frequency=bwem").as_str()
            },
        }
        self
    }
    pub fn aggregation_method(&mut self, order: ObservationAggregationMethod) -> &mut ObservationBuilder {
        match order {
            ObservationAggregationMethod::SUM => {
                self.option_string += format!("&aggregation_method=sum").as_str()
            },
            ObservationAggregationMethod::EOP => {
                self.option_string += format!("&aggregation_method=eop").as_str()
            },
            _ => () // AVG is the default so do nothing
        }
        self
    }
    pub fn output_type(&mut self, order: ObservationOutputType) -> &mut ObservationBuilder {
        match order {
            ObservationOutputType::VDALL => {
                self.option_string += format!("&output_type=2").as_str()
            },
            ObservationOutputType::VDNEW => {
                self.option_string += format!("&output_type=3").as_str()
            },
            ObservationOutputType::INITIAL => {
                self.option_string += format!("&output_type=4").as_str()
            },
            _ => () // AVG is the default so do nothing
        }
        self
    }
    pub fn vintage_date(&mut self, date: &str) -> &mut ObservationBuilder {
        if self.vintage_dates.len() != 0 {
            self.vintage_dates.push(',');
        } 
        self.vintage_dates += date;

        self
    }
}