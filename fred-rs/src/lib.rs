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

/// fred/category endpoints
pub mod category;

/// fred/releases endpoints
pub mod releases;

/// fred/series endpoints
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

/// fred/tags endpoints
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
/// builder.realtime_start("2000-01-01");
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


/// fred/related_tags endpoint
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::related_tags::{Builder, Response, SortOrder, OrderBy};
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

/// fred/sources endpoint
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::sources::{Builder, Response, SortOrder};
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

/// fred/source endpoint
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