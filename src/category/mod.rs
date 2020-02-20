
/// fred/category/children endpoint
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::category::Response;
/// 
/// let mut c = match FredClient::new() {
/// Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let resp: Response = match c.category_children(125) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for s in resp.categories {
///     println!("ID: {}  Name: {}  ParentID: {}", s.id, s.name, s.parent_id);
/// }
/// ```
pub mod children;

/// fred/category/children endpoint
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::category::Response;
/// 
/// let mut c = match FredClient::new() {
/// Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let resp: Response = match c.category_related(125) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for s in resp.categories {
///     println!("ID: {}  Name: {}  ParentID: {}", s.id, s.name, s.parent_id);
/// }
/// ```
pub mod related;

/// Get the series in a category
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::category::series::{Builder, OrderBy, SortOrder};
/// use fred_rs::series::Response;
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
///     .sort_order(SortOrder::Descending)
///     .order_by(OrderBy::Frequency);
/// 
/// let resp: Response = match c.category_series(125, Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.seriess {
///     println!(
///         "{}: {} {}",
///         item.id,
///         item.title,
///         item.popularity,
///     );
/// }
/// ```
pub mod series;


/// Get the FRED tags for a category
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::category::tags::{Builder, OrderBy, SortOrder};
/// use fred_rs::tags::Response;
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
///     .sort_order(SortOrder::Descending)
///     .order_by(OrderBy::Name);
/// 
/// let resp: Response = match c.category_tags(125, Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.tags {
///     println!(
///         "{}: {} {}",
///         item.name,
///         item.series_count,
///         item.popularity
///     );
/// }
/// ```
pub mod tags;

/// Get the related FRED tags for one or more FRED tags within a category
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::category::related_tags::{Builder, OrderBy, SortOrder};
/// use fred_rs::tags::Response;
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
///     .tag_name("usa")
///     .limit(5)
///     .sort_order(SortOrder::Descending)
///     .order_by(OrderBy::Name);
/// 
/// let resp: Response = match c.category_related_tags(125, builder) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.tags {
///     println!(
///         "{}: {} {}",
///         item.name,
///         item.series_count,
///         item.popularity
///     );
/// }
/// ```
pub mod related_tags;

// -----------------------------------------------------------------------------

use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/category endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category.html] (https://research.stlouisfed.org/docs/api/fred/category.html)
pub struct Response {
    /// Series returned by the search
    pub categories: Vec<Category>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular tag
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category.html](https://research.stlouisfed.org/docs/api/fred/category.html)
pub struct Category {
    /// The source ID
    pub id: usize,
    /// The source name
    pub name: String,
    /// A link to the source's website
    pub parent_id: usize,
    /// Additional information about the category
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn category_no_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let resp: Response = match c.category(125) {
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