#![crate_name = "fred_rs"]

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