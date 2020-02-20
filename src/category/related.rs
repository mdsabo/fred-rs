
use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/category endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category_children.html] (https://research.stlouisfed.org/docs/api/fred/category_children.html)
pub struct Response {
    /// Series returned by the search
    pub categories: Vec<Category>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular tag
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category_children.html](https://research.stlouisfed.org/docs/api/fred/category_children.html)
pub struct Category {
    /// The source ID
    pub id: usize,
    /// The source name
    pub name: String,
    /// A link to the source's website
    pub parent_id: usize,
}

pub struct Builder {
    option_string: String,
}

impl Builder {

    /// Initializes a new category::related::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not check for duplicate arguments and instead adds all arguments to the URL string.  The FRED API behavior for duplicates in unknown.
    /// 
    /// ```
    /// use fred_rs::category::related::Builder;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn category_related_no_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let resp: Response = match c.category_related(125) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for s in resp.categories {
            println!("ID: {}  Name: {}  ParentID: {}", s.id, s.name, s.parent_id);
        }
    } 
}