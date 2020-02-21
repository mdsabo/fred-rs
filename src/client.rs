//! Functions and definitons related to the persistent client
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::series::observation::{Builder, Units, Frequency, Response};
//! 
//! // Create the client object
//! let mut c = match FredClient::new() {
//!     Ok(c) => c,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         return
//!     },
//! };
//! 
//! // Create the argument builder
//! let mut builder = Builder::new();
//! 
//! // Set the arguments for the builder
//! builder
//!     .observation_start("2000-01-01")
//!     .units(Units::PCH)
//!     .frequency(Frequency::M);
//! 
//! // Make the request and pass in the builder to apply the arguments
//! let resp: Response = match c.series_observation("GNPCA", Some(builder)) {
//!     Ok(resp) => resp,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         return
//!     },
//! };
//! ```

use reqwest::blocking::{Client, Response};

use std::time::Duration;
use std::env;

use crate::*;

const FRED_BASE_URL: &str = "https://api.stlouisfed.org/fred/";
const FRED_API_KEY: &str = "FRED_API_KEY";

#[derive(Clone, Debug)]
/// Persistent client object used to access the FRED API
/// 
/// Each method for the client represents a data endpoint provided by the API and will return a data object representing the response contents.
pub struct FredClient {
    client: Client,
    url_base: &'static str,
    api_key: String,
}

impl FredClient {

    /// Creates and initializes a new client object
    /// 
    /// The client will attempt to load an API key from the environment variable 'FRED_API_KEY'.  If this variable is undefined, the key remains empty.
    /// 
    /// If a connection cannot be made to the FRED API, it returns Err containing an error message.
    /// 
    /// ```
    /// use fred_rs::client::FredClient;
    /// 
    /// let mut client = match FredClient::new() {
    ///     Ok(c) => c,
    ///     Err(msg) => {
    ///         println!("{}", msg);
    ///         return
    ///     },
    /// };
    /// ```
    pub fn new() -> Result<FredClient, String> {

        let client = match Client::builder().timeout(Duration::from_secs(30)).build() {
            Ok(c) => c,
            Err(msg) => return Err(msg.to_string()),
        };

        let api_key = match env::var(FRED_API_KEY) {
            Ok(val) => val,
            Err(_) => String::from(""),
        };

        let fred = FredClient {
            client,
            url_base: FRED_BASE_URL,
            api_key,
        };

        let url = format!("{}category?category_id=125&api_key={}&file_type=json", fred.url_base, fred.api_key);
        match fred.client.get(url.as_str()).send() {
            Ok(_) => (),
            Err(msg) => return Err(msg.to_string()),
        }

        return Ok(fred)

    }

    /// Sets the FRED API key for the client
    /// 
    /// # Arguments
    /// * `key` - The [API key](https://research.stlouisfed.org/docs/api/api_key.html) generated to access FRED 
    /// 
    /// ```
    /// use fred_rs::client::FredClient;
    /// 
    /// let mut client = match FredClient::new() {
    ///     Ok(c) => c,
    ///     Err(msg) => {
    ///         println!("{}", msg);
    ///         return
    ///     },
    /// };
    /// 
    /// client.with_key("abcdefghijklmnopqrstuvwxyz123456");
    /// ```
    pub fn with_key(&mut self, key: &str) {
        self.api_key = String::from(key);
    }

    fn get_request(&mut self, url: &str) -> Result<Response, String> {
        match self.client.get(url).send() {
            Ok(r) => Ok(r),
            Err(msg) => Err(msg.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Series

    /// [See fred_rs::series](../series/index.html)
    /// 
    /// # Arguments
    /// `series_id` - The id for a series [[Link]](https://research.stlouisfed.org/docs/api/fred/series.html#series_id)
    pub fn series(
        &mut self,
        series_id: &str,
        builder: Option<series::Builder>
    ) -> Result<series::Response, String> {
        let mut url: String = format!(
            "{}series?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::series::categories](../series/categories/index.html)
    /// 
    /// # Arguments
    /// `series_id` - The id for a series [[Link]](https://research.stlouisfed.org/docs/api/fred/series_categories.html#series_id)
    pub fn series_categories(
        &mut self,
        series_id: &str,
        builder: Option<series::categories::Builder>
    ) -> Result<category::Response, String> {
        let mut url: String = format!(
            "{}series/categories?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::series::observation](../series/observation/index.html)
    /// 
    /// # Arguments
    /// `series_id` - The id for a series [[Link]](https://research.stlouisfed.org/docs/api/fred/series_observation.html#series_id)
    pub fn series_observation(
        &mut self,
        series_id: &str,
        builder: Option<series::observation::Builder>
    ) -> Result<series::observation::Response, String> {
        let mut url: String = format!(
            "{}series/observations?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::series::release](../series/release/index.html)
    /// 
    /// # Arguments
    /// `series_id` - The id for a series [[Link]](https://research.stlouisfed.org/docs/api/fred/series_release.html#series_id)
    pub fn series_release(
        &mut self,
        series_id: &str,
        builder: Option<series::release::Builder>
    ) -> Result<release::Response, String> {
        let mut url: String = format!(
            "{}series/release?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }
    
    /// [See fred_rs::series::tags](../series/tags/index.html)
    /// 
    /// # Arguments
    /// `series_id` - The id for a series [[Link]](https://research.stlouisfed.org/docs/api/fred/series_tags.html#series_id)
    pub fn series_tags(
        &mut self,
        series_id: &str,
        builder: Option<series::tags::Builder>
    ) -> Result<tags::Response, String> {

        let mut url: String = format!(
            "{}series/tags?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }
        
        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::series::updates](../series/updates/index.html)
    pub fn series_updates(
        &mut self,
        builder: Option<series::updates::Builder>
    ) -> Result<series::updates::Response, String> {

        let mut url: String = format!(
            "{}series/updates?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }
        
        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::series::vintagedates](../series/vintagedates/index.html)
    /// 
    /// # Arguments
    /// `series_id` - The id for a series [[Link]](https://research.stlouisfed.org/docs/api/fred/series_vintagedates.html#series_id)
    pub fn series_vintagedates(
        &mut self,
        series_id: &str,
        builder: Option<series::vintagedates::Builder>
    ) -> Result<series::vintagedates::Response, String> {

        let mut url: String = format!(
            "{}series/vintagedates?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }
        
        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Series/Search

    /// [See fred_rs::series::search](../series/search/index.html)
    /// 
    /// # Arguments
    /// `search_text` - The words to match against economic data series [[Link]](https://research.stlouisfed.org/docs/api/fred/series_search.html#search_text)
    pub fn series_search(
        &mut self,
        search_text: &str,
        builder: Option<series::search::Builder>
    ) -> Result<series::Response, String> {
        let search_text = search_text.replace(" ", "%20"); // encode strings in url

        let mut url: String = format!(
            "{}series/search?search_text={}&api_key={}&file_type=json",
            self.url_base,
            search_text,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::series::search::tags](../series/search/tags/index.html)
    /// 
    /// # Arguments
    /// `series_search_text` - The words to match against economic data series [[Link]](https://research.stlouisfed.org/docs/api/fred/series_search_tags.html#search_text)
    pub fn series_search_tags(
        &mut self,
        series_search_text: &str,
        builder: Option<series::search::tags::Builder>
    ) -> Result<tags::Response, String> {
        let search_text = series_search_text.replace(" ", "%20"); // encode spaces in url

        let mut url: String = format!(
            "{}series/search/tags?series_search_text={}&api_key={}&file_type=json",
            self.url_base,
            search_text,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }
        
        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::series::search::related_tags](../series/search/related_tags/index.html)
    /// 
    /// # Arguments
    /// `series_search_text` - The words to match against economic data series [[Link]](https://research.stlouisfed.org/docs/api/fred/series_search_related_tags.html#search_text)
    pub fn series_search_related_tags(
        &mut self,
        series_search_text: &str,
        builder: series::search::related_tags::Builder
    ) -> Result<tags::Response, String> {

        let search_text = series_search_text.replace(" ", "%20"); // encode spaces in url

        let mut url: String = format!(
            "{}series/search/related_tags?series_search_text={}&api_key={}&file_type=json",
            self.url_base,
            search_text,
            self.api_key
        );

        match builder.build() {
            Ok(s) => url.push_str(s.as_str()),
            Err(msg) => return Err(msg),
        }
                
        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Tags

    /// [See fred_rs::tags](../tags/index.html)
    pub fn tags(
        &mut self,
        builder: Option<tags::Builder>
    ) -> Result<tags::Response, String> {
        let mut url: String = format!(
            "{}tags?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::tags::series](../tags/series/index.html)
    pub fn tags_series(
        &mut self,
        builder: tags::series::Builder
    ) -> Result<series::Response, String> {
        let mut url: String = format!(
            "{}tags/series?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder.build() {
            Ok(opt) => url.push_str(opt.as_str()),
            Err(msg) => return Err(msg),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Related Tags

    /// [See fred_rs::related_tags](../related_tags/index.html)
    pub fn related_tags(
        &mut self,
        builder: related_tags::Builder
    ) -> Result<tags::Response, String> {
        let mut url: String = format!(
            "{}related_tags?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder.build() {
            Ok(opt) => url.push_str(opt.as_str()),
            Err(msg) => return Err(msg),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Sources

    /// [See fred_rs::sources](../sources/index.html)
    pub fn sources(
        &mut self,
        builder: Option<sources::Builder>
    ) -> Result<source::Response, String> {
        let mut url: String = format!(
            "{}sources?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Source

    /// [See fred_rs::source](../source/index.html)
    /// 
    /// # Arguments
    /// `source_id` - The id for a source [[Link]](https://research.stlouisfed.org/docs/api/fred/source.html#source_id)
    pub fn source(
        &mut self,
        source_id: usize,
        builder: Option<source::Builder>
    ) -> Result<source::Response, String> {
        let mut url: String = format!(
            "{}source?source_id={}&api_key={}&file_type=json",
            self.url_base,
            source_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::source::releases](../source/releases/index.html)
    /// 
    /// # Arguments
    /// `source_id` - The id for a source [[Link]](https://research.stlouisfed.org/docs/api/fred/source_releases.html#source_id)
    pub fn source_releases(
        &mut self,
        source_id: usize,
        builder: Option<source::releases::Builder>
    ) -> Result<release::Response, String> {
        let mut url: String = format!(
            "{}source/releases?source_id={}&api_key={}&file_type=json",
            self.url_base,
            source_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Category

    /// [See fred_rs::category](../category/index.html)
    /// 
    /// # Arguments
    /// `category_id` - The id for a category [[Link]](https://research.stlouisfed.org/docs/api/fred/category.html#category_id)
    pub fn category(
        &mut self,
        category_id: usize
    ) -> Result<category::Response, String> {
        let url: String = format!(
            "{}category?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::category::children](../category/children/index.html)
    /// 
    /// # Arguments
    /// `category_id` - The id for a category [[Link]](https://research.stlouisfed.org/docs/api/fred/category_children.html#category_id)
    pub fn category_children(
        &mut self,
        category_id: usize,
        builder: Option<category::children::Builder>,
    ) -> Result<category::Response, String> {
        let mut url: String = format!(
            "{}category/children?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::category::related](../category/related/index.html)
    /// 
    /// # Arguments
    /// `category_id` - The id for a category [[Link]](https://research.stlouisfed.org/docs/api/fred/category_related.html#category_id)
    pub fn category_related(
        &mut self,
        category_id: usize,
        builder: Option<category::related::Builder>,
    ) -> Result<category::Response, String> {
        let mut url: String = format!(
            "{}category/related?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::category::series](../category/series/index.html)
    /// 
    /// # Arguments
    /// `category_id` - The id for a category [[Link]](https://research.stlouisfed.org/docs/api/fred/series.html#category_id)
    pub fn category_series(
        &mut self,
        category_id: usize,
        builder: Option<category::series::Builder>
    ) -> Result<series::Response, String> {
        let mut url: String = format!(
            "{}category/series?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::category::tags](../category/tags/index.html)
    /// 
    /// # Arguments
    /// `category_id` - The id for a category [[Link]](https://research.stlouisfed.org/docs/api/fred/category_tags.html#category_id)
    pub fn category_tags(
        &mut self,
        category_id: usize,
        builder: Option<category::tags::Builder>
    ) -> Result<tags::Response, String> {
        let mut url: String = format!(
            "{}category/tags?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::category::related_tags](../category/related_tags/index.html)
    /// 
    /// # Arguments
    /// `category_id` - The id for a category [[Link]](https://research.stlouisfed.org/docs/api/fred/category_related_tags.html#category_id)
    pub fn category_related_tags(
        &mut self,
        category_id: usize,
        builder: category::related_tags::Builder
    ) -> Result<tags::Response, String> {
        let mut url: String = format!(
            "{}category/related_tags?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

        match builder.build() {
            Ok(o) => url.push_str(o.as_str()),
            Err(msg) => return Err(msg),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Releases

    /// [See fred_rs::releases](../releases/index.html)
    pub fn releases(
        &mut self,
        builder: Option<releases::Builder>
    ) -> Result<release::Response, String> {
        let mut url: String = format!(
            "{}releases?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::releases::dates](../releases/dates/index.html)
    pub fn releases_dates(
        &mut self,
        builder: Option<releases::dates::Builder>
    ) -> Result<releases::dates::Response, String> {
        let mut url: String = format!(
            "{}releases/dates?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Release

    /// [See fred_rs::release](../release/index.html)
    /// 
    /// # Arguments
    /// `release_id` - The id for a release [[Link]](https://research.stlouisfed.org/docs/api/fred/release.html#release_id)
    pub fn release(
        &mut self,
        release_id: usize,
        builder: Option<release::Builder>
    ) -> Result<release::Response, String> {
        let mut url: String = format!(
            "{}release?release_id={}&api_key={}&file_type=json",
            self.url_base,
            release_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::release::series](../release/series/index.html)
    /// 
    /// # Arguments
    /// `release_id` - The id for a release [[Link]](https://research.stlouisfed.org/docs/api/fred/release_series.html#release_id)
    pub fn release_series(
        &mut self,
        release_id: usize,
        builder: Option<release::series::Builder>
    ) -> Result<series::Response, String> {
        let mut url: String = format!(
            "{}release/series?release_id={}&api_key={}&file_type=json",
            self.url_base,
            release_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::release::sources](../release/sources/index.html)
    /// 
    /// # Arguments
    /// `release_id` - The id for a release [[Link]](https://research.stlouisfed.org/docs/api/fred/release_sources.html#release_id)
    pub fn release_sources(
        &mut self,
        release_id: usize,
        builder: Option<release::sources::Builder>
    ) -> Result<source::Response, String> {
        let mut url: String = format!(
            "{}release/sources?release_id={}&api_key={}&file_type=json",
            self.url_base,
            release_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::release::tags](../release/tags/index.html)
    /// 
    /// # Arguments
    /// `release_id` - The id for a release [[Link]](https://research.stlouisfed.org/docs/api/fred/release_tags.html#release_id)
    pub fn release_tags(
        &mut self,
        release_id: usize,
        builder: Option<release::tags::Builder>
    ) -> Result<tags::Response, String> {
        let mut url: String = format!(
            "{}release/tags?release_id={}&api_key={}&file_type=json",
            self.url_base,
            release_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::release::related_tags](../release/related_tags/index.html)
    /// 
    /// # Arguments
    /// `release_id` - The id for a release [[Link]](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#release_id)
    pub fn release_related_tags(
        &mut self,
        release_id: usize,
        builder: release::related_tags::Builder
    ) -> Result<tags::Response, String> {
        let mut url: String = format!(
            "{}release/related_tags?release_id={}&api_key={}&file_type=json",
            self.url_base,
            release_id,
            self.api_key
        );

        match builder.build() {
            Ok(o) => url.push_str(o.as_str()),
            Err(msg) => return Err(msg),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    /// [See fred_rs::release::tables](../release/tables/index.html)
    /// 
    /// # Arguments
    /// `release_id` - The id for a release [[Link]](https://research.stlouisfed.org/docs/api/fred/release_tables.html#release_id)
    pub fn release_tables(
        &mut self,
        release_id: usize,
        builder: Option<release::tables::Builder>
    ) -> Result<release::tables::Response, String> {
        let mut url: String = format!(
            "{}release/tables?release_id={}&api_key={}&file_type=json",
            self.url_base,
            release_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.build().as_str()),
            None => (),
        }
        
        match self.get_request(url.as_str()) {
            Ok(resp) => {
                let text = resp.text().unwrap();
                match serde_json::from_str(&text) {
                    Ok(val) => Ok(val),
                    Err(_e) => {
                        match serde_json::from_str(&text) {
                            Ok(e) => {
                                let err: error::FredError = e;
                                let err_msg = format!(
                                    "ERROR {}: {}",
                                    err.error_code,
                                    err.error_message
                                );
                                return Err(err_msg);
                            },
                            Err(msg) => return Err(String::from(msg.to_string())),
                        }
                    },
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_new() {
        match FredClient::new() {
            Ok(_) => assert_eq!(1, 1),
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1)
            },
        }
    }
}