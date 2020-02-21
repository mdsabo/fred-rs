#![crate_name = "fred_rs"]

//! **fred-rs** is a simple interface for accessing the Federal Reserve Bank of St. Louis's FRED API.
//! 
//! # FRED (Federal Reserve Economic Data)
//! FRED is a large online database of economic data hosted by the Federal
//! Reserve Bank of St. Louis.  The website currently hosts approximately 
//! "672,000 US and international time series from 89 sources."
//! 
//! Access to the raw data is available through the FRED API.  **fred-rs** is 
//! an intermediate layer between the HTTPS client and the user 
//! application.  Requests to the FRED API are made through structured calls 
//! to the fred_rs FredClient and data is returned as usable data objects 
//! (structs).
//! 
//! # Usage
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
//! 
//! ### Request Parameters
//! All endpoints use the builder approach to construct the API URL.  Each 
//! builder method corresponds to a paramter that can be added to the API 
//! request. 
//! 
//! In the example above, three parameters are added to the request, 
//! observation_start, units and frequency.  The [FRED API Documentation](https://research.stlouisfed.org/docs/api/fred/#General_Documentation) 
//! explains the possible parameters for each endpoint.  Required paramters 
//! (except the `tag_names` paramter) are passed to the client function 
//! itself.  In the example, series_id is a required paramter and is passed 
//! directly to the client function as `"GNPCA"`.  The `tag_names` parameter 
//! available on some endpoints accepts a list of arguments, so it is easier to
//!  pass this argument to the builder.
//! 
//! # API Key
//! Developers need to request an API Key in order to access FRED.  This 
//! can be done at [https://research.stlouisfed.org/docs/api/api_key.html](https://research.stlouisfed.org/docs/api/api_key.html).
//! 
//! **fred-rs** looks for the `FRED_API_KEY` environment variable by 
//! default.  The environment variable can be set with the following 
//! line in Bash.
//! ```bash
//! export FRED_API_KEY=abcdefghijklmnopqrstuvwxyz123456
//! ```
//! 
//! Alternatively, the `FredClient.with_key()` function allows the key to be 
//! set from a string reference.
//! ```rust
//! use fred_rs::client::FredClient;
//! 
//! let mut client = match FredClient::new() {
//!     Ok(c) => c,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         return
//!     },
//! };
//! 
//! client.with_key("abcdefghijklmnopqrstuvwxyz123456");
//! ```
//! 
//! # Issues/Bugs/Improvments/Help/Questions
//! If you discover any issues or bugs, want to suggest any improvements, or 
//! have questions about the crate, feel free to open a GitHub issue or email 
//! me directly at [matthewdsabo@gmail.com](mailto:matthewdsabo@gmail.com) with 
//! **fred-rs** in the subject line.
//! 
//! #### License
//! 
//! Licensed under either of Apache License, Version
//! 2.0 or MIT license at your option.
//! 
//! Unless you explicitly state otherwise, any contribution intentionally 
//! submitted for inclusion in this crate by you, as defined in the Apache-2.0 
//! license, shall be dual licensed as above, without any additional terms or 
//! conditions.

/// Functions and definitons related to the persistent client
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::observation::{Builder, Units, Frequency, Response};
/// 
/// // Create the client object
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         return
///     },
/// };
/// 
/// // Create the argument builder
/// let mut builder = Builder::new();
/// 
/// // Set the arguments for the builder
/// builder
///     .observation_start("2000-01-01")
///     .units(Units::PCH)
///     .frequency(Frequency::M);
/// 
/// // Make the request and pass in the builder to apply the arguments
/// let resp: Response = match c.series_observation("GNPCA", Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         return
///     },
/// };
/// ```
pub mod client;

/// Get a category
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category.html](https://research.stlouisfed.org/docs/api/fred/category.html)
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
/// let resp: Response = match c.category(125) {
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
pub mod category;

/// Get all releases of economic data
/// 
/// [https://research.stlouisfed.org/docs/api/fred/releases.html](https://research.stlouisfed.org/docs/api/fred/releases.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::releases::{Builder, SortOrder, OrderBy};
/// use fred_rs::release::Response;
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
/// let mut builder = Builder::new();
/// builder
///     .limit(5)
///     .sort_order(SortOrder::Ascending)
///     .order_by(OrderBy::ReleaseId);
/// 
/// let resp: Response = match c.releases(Some(builder)) {
/// Ok(resp) => resp,
/// Err(msg) => {
///     println!("{}", msg);
///     assert_eq!(2, 1);
///     return
///     },
/// };
/// 
/// for item in resp.releases {
///     println!("{}: {}", item.id, item.name);
/// }
/// ```
pub mod releases;

/// Get a release of economic data
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release.html](https://research.stlouisfed.org/docs/api/fred/release.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::release::{Builder, Response};
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
/// let mut builder = Builder::new();
/// builder
///     .realtime_start("2000-01-01");
/// 
/// let resp: Response = match c.release(9, Some(builder)) {
/// Ok(resp) => resp,
/// Err(msg) => {
///     println!("{}", msg);
///     assert_eq!(2, 1);
///     return
///     },
/// };
/// 
/// for item in resp.releases {
///     println!("{}: {}", item.id, item.name);
/// }
/// ```
pub mod release;

/// Get an economic data series
/// 
/// [https://research.stlouisfed.org/docs/api/fred/series.html](https://research.stlouisfed.org/docs/api/fred/series.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::series::{Builder, Response};
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
/// builder.realtime_start("2000-01-01");
/// 
/// let resp: Response = match c.series("UNRATE", Some(builder)) {
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
///         "{}: {} {} {}",
///         item.id,
///         item.title,
///         item.realtime_start,
///         item.realtime_end
///     );
/// }
/// ```
pub mod series;

/// Get all tags, search for tags, or get tags by name
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags.html](https://research.stlouisfed.org/docs/api/fred/tags.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::tags::{Builder, Response, SortOrder, OrderBy};
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
///     .sort_order(SortOrder::Descending)
///     .order_by(OrderBy::Created)
///     .limit(5);
/// 
/// let resp: Response = match c.tags(Some(builder)) {
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
///         "{}: {}",
///         item.name,
///         item.created
///     );
/// }
/// ```
pub mod tags;


/// Get the related tags for one or more tags
/// 
/// [https://research.stlouisfed.org/docs/api/fred/related_tags.html](https://research.stlouisfed.org/docs/api/fred/related_tags.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::related_tags::{Builder, SortOrder, OrderBy};
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
///     .order_by(OrderBy::Popularity);
/// 
/// let resp: Response = match c.related_tags(builder) {
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
///         "{}: {}",
///         item.name,
///         item.created
///     );
/// }
/// ```
pub mod related_tags;

/// Get all sources of economic data
/// 
/// [https://research.stlouisfed.org/docs/api/fred/sources.html](https://research.stlouisfed.org/docs/api/fred/sources.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::sources::{Builder, SortOrder};
/// use fred_rs::source::Response;
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
///     .sort_order(SortOrder::Descending);
/// 
/// let resp: Response = match c.sources(Some(builder)) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.sources {
///     match item.link {
///         Some(l) => println!("{}: {}", item.name, l),
///         None => println!("{}: null", item.name),
///     }
/// }
/// ```
pub mod sources;

/// Get a source of economic data
/// 
/// [https://research.stlouisfed.org/docs/api/fred/source.html](https://research.stlouisfed.org/docs/api/fred/source.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::source::Response;
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
/// let resp: Response = match c.source(1, None) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.sources {
///     match item.link {
///         Some(l) => println!("{}: {}", item.name, l),
///         None => println!("{}: null", item.name),
///     }
/// }
/// ```
pub mod source;

mod error;