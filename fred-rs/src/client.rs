
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
    
    pub fn series_tags(
        &mut self,
        series_id: &str,
        builder: Option<series::tags::Builder>
    ) -> Result<series::tags::Response, String> {

        let mut url: String = format!(
            "{}series/tags?series_id={}&api_key={}&file_type=json",
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

    // ----------------------------------------------------------------------
    // Series/Search
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

    // ----------------------------------------------------------------------
    // Tags

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

    pub fn tags_series(
        &mut self,
        builder: tags::series::Builder
    ) -> Result<tags::series::Response, String> {
        let mut url: String = format!(
            "{}tags/series?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder.options() {
            Ok(opt) => url.push_str(opt.as_str()),
            Err(msg) => return Err(msg),
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

    // ----------------------------------------------------------------------
    // Related Tags

    pub fn related_tags(
        &mut self,
        builder: related_tags::Builder
    ) -> Result<related_tags::Response, String> {
        let mut url: String = format!(
            "{}related_tags?api_key={}&file_type=json",
            self.url_base,
            self.api_key
        );

        match builder.options() {
            Ok(opt) => url.push_str(opt.as_str()),
            Err(msg) => return Err(msg),
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

    // ----------------------------------------------------------------------
    // Sources

    pub fn sources(
        &mut self,
        builder: Option<sources::Builder>
    ) -> Result<sources::Response, String> {
        let mut url: String = format!(
            "{}sources?api_key={}&file_type=json",
            self.url_base,
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

    // ----------------------------------------------------------------------
    // Source

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

    pub fn source_releases(
        &mut self,
        source_id: usize,
        builder: Option<source::releases::Builder>
    ) -> Result<source::releases::Response, String> {
        let mut url: String = format!(
            "{}source/releases?source_id={}&api_key={}&file_type=json",
            self.url_base,
            source_id,
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

    // ----------------------------------------------------------------------
    // Category

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
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn category_children(
        &mut self,
        category_id: usize
    ) -> Result<category::children::Response, String> {
        let url: String = format!(
            "{}category/children?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

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

    pub fn category_related(
        &mut self,
        category_id: usize
    ) -> Result<category::related::Response, String> {
        let url: String = format!(
            "{}category/related?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

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

    pub fn category_series(
        &mut self,
        category_id: usize,
        builder: Option<category::series::Builder>
    ) -> Result<category::series::Response, String> {
        let mut url: String = format!(
            "{}category/series?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
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

    pub fn category_tags(
        &mut self,
        category_id: usize,
        builder: Option<category::tags::Builder>
    ) -> Result<category::tags::Response, String> {
        let mut url: String = format!(
            "{}category/tags?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

        match builder {
            Some(b) => url.push_str(b.options().as_str()),
            None => (),
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

    pub fn category_related_tags(
        &mut self,
        category_id: usize,
        builder: category::related_tags::Builder
    ) -> Result<category::related_tags::Response, String> {
        let mut url: String = format!(
            "{}category/related_tags?category_id={}&api_key={}&file_type=json",
            self.url_base,
            category_id,
            self.api_key
        );

        match builder.options() {
            Ok(o) => url.push_str(o.as_str()),
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

    // ----------------------------------------------------------------------
    // Releases

    pub fn releases(
        &mut self,
        builder: Option<releases::Builder>
    ) -> Result<releases::Response, String> {
        let mut url: String = format!(
            "{}releases?api_key={}&file_type=json",
            self.url_base,
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