
// MIT License
// 
// Copyright (c) 2020 Matthew Sabo
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Get the releases for a source
/// 
/// [https://research.stlouisfed.org/docs/api/fred/source_releases.html](https://research.stlouisfed.org/docs/api/fred/source_releases.html
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::source::releases::{Builder, SortOrder, OrderBy};
/// use fred_rs::release::Response;
/// 
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
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
///     .order_by(OrderBy::Name)
///     .sort_order(SortOrder::Descending);
/// 
/// let resp: Response = match c.source_releases(1, Some(builder)) {
///     Ok(resp) => resp,
///         Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.releases {
///     match item.link {
///         Some(l) => println!("{}: {}", item.name, l),
///         None => println!("{}: No Link", item.name),
///     }
/// }
/// ```
pub mod releases;

// -----------------------------------------------------------------------------

use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/source endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/source.html] (https://research.stlouisfed.org/docs/api/fred/source.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// How the results are ordered
    pub order_by: Option<String>,
    // Results are listed in ascending or descending
    pub sort_order: Option<String>,
    /// Number of results returned
    pub count: Option<usize>,
    /// ???
    pub offset: Option<usize>,
    /// Maximum number of results to return
    pub limit: Option<usize>,
    /// Series returned by the search
    pub sources: Vec<Source>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular tag
/// 
/// [https://research.stlouisfed.org/docs/api/fred/source.html](https://research.stlouisfed.org/docs/api/fred/source.html)
pub struct Source {
    /// The source ID
    pub id: usize,
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// The source name
    pub name: String,
    /// A link to the source's website
    pub link: Option<String>,
    /// Additional notes about the source
    pub notes: Option<String>
}

pub struct Builder {
    option_string: String,
}

impl Builder {

    /// Initializes a new sources::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
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
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/source.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/source.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/source.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/source.html#realtime_end)
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn source_no_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let resp: Response = match c.source(1, None) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for s in resp.sources {
            match s.link {
                Some(l) => println!("{}: {}", s.name, l),
                None => println!("{}", s.name)
            }
        }
    } 
}