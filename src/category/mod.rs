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

/// Get the child categories for a specified parent category
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category_children.html](https://research.stlouisfed.org/docs/api/fred/category_children.html)
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

/// Get the related categories for a category
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category_related.html](https://research.stlouisfed.org/docs/api/fred/category_related.html)
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
/// [https://research.stlouisfed.org/docs/api/fred/category_series.html](https://research.stlouisfed.org/docs/api/fred/category_series.html)
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


/// Get the tags for a category
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category_tags.html](https://research.stlouisfed.org/docs/api/fred/category_tags.html)
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

/// Get the related tags for a category
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category_related_tags.html](https://research.stlouisfed.org/docs/api/fred/category_related_tags.html)
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
/// Response data structure for a collection of categories
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category.html] (https://research.stlouisfed.org/docs/api/fred/category.html)
pub struct Response {
    /// List of categories returned by the query
    pub categories: Vec<Category>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular category
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category.html](https://research.stlouisfed.org/docs/api/fred/category.html)
pub struct Category {
    /// The category ID number
    pub id: usize,
    /// The category name
    pub name: String,
    /// The parent ID number of the category
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