
pub use reqwest::blocking::{Client, Response};

use std::time::Duration;
use std::env;

use crate::*;

const FRED_BASE_URL: &str = "https://api.stlouisfed.org/fred/";
const FRED_API_KEY: &str = "FRED_API_KEY";

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
    /// If a connection cannot be made to the FRED API, it returns Err containing an error message.
    /// 
    /// ```
    /// use fred_rs::client::FredClient;
    /// 
    /// let mut c = match FredClient::new() {
    ///     Ok(c) => c,
    ///     Err(msg) => {
    ///         println!("{}", msg);
    ///         return
    ///     },
    /// };
    /// ```
    pub fn new() -> Result<FredClient, String> {

        let client = match Client::builder().timeout(Duration::from_secs(10)).build() {
            Ok(c) => c,
            Err(msg) => return Err(msg.to_string()),
        };

        let api_key = match env::var(FRED_API_KEY) {
            Ok(val) => val,
            Err(_) => return Err(String::from("FRED_API_KEY not found.")),
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

    fn get_request(&mut self, url: &str) -> Result<Response, String> {
        match self.client.get(url).send() {
            Ok(r) => Ok(r),
            Err(msg) => Err(msg.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Series

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
            Some(b) => url.push_str(b.options().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn series_categories(
        &mut self,
        series_id: &str,
        builder: Option<series::categories::Builder>
    ) -> Result<series::categories::Response, String> {
        let mut url: String = format!(
            "{}series/categories?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.options().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

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
            Some(b) => url.push_str(b.options().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn series_release(
        &mut self,
        series_id: &str,
        builder: Option<series::release::Builder>
    ) -> Result<series::release::Response, String> {
        let mut url: String = format!(
            "{}series/release?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.options().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn series_search(
        &mut self,
        search_text: &str,
        builder: Option<series::search::Builder>
    ) -> Result<series::search::Response, String> {
        let search_text = search_text.replace(" ", "%20"); // encode strings in url

        let mut url: String = format!(
            "{}series/search?search_text={}&api_key={}&file_type=json",
            self.url_base,
            search_text,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.options().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn series_search_tags(
        &mut self,
        series_search_text: &str,
        builder: Option<series::search::tags::Builder>
    ) -> Result<series::search::tags::Response, String> {
        let search_text = series_search_text.replace(" ", "%20"); // encode spaces in url

        let mut url: String = format!(
            "{}series/search/tags?series_search_text={}&api_key={}&file_type=json",
            self.url_base,
            search_text,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.options().as_str()),
            None => (),
        }
        
        match self.get_request(url.as_str()) {
            Ok(resp) => {
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn series_search_related_tags(
        &mut self,
        series_search_text: &str,
        builder: series::search::related_tags::Builder
    ) -> Result<series::search::related_tags::Response, String> {

        let search_text = series_search_text.replace(" ", "%20"); // encode spaces in url

        let mut url: String = format!(
            "{}series/search/related_tags?series_search_text={}&api_key={}&file_type=json",
            self.url_base,
            search_text,
            self.api_key
        );

        match builder.options() {
            Ok(s) => url.push_str(s.as_str()),
            Err(msg) => return Err(msg),
        }

        println!("{}", url);
        
        match self.get_request(url.as_str()) {
            Ok(resp) => {
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
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

    #[test]
    fn series_observation() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let resp: series::observation::Response = match c.series_observation("GNPCA", None) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        /*for item in resp.observations {
            println!("{}: {}", item.date, item.value.parse::<f64>().unwrap());
        }*/
        assert_eq!(resp.observations[0].value, String::from("1120.076"));
    }

    #[test]
    fn series_observation_with_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let mut opt_builder = series::observation::Builder::new();
        opt_builder
            .observation_start("2000-01-01")
            .units(series::observation::Units::PCH)
            .frequency(series::observation::Frequency::M);

        let resp: series::observation::Response = match c.series_observation("UNRATE", Some(opt_builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.observations {
            println!("{}: {}", item.date, item.value.parse::<f64>().unwrap());
        }
        //assert_eq!(resp.observations[0].value, String::from("1120.076"));
    } 
}