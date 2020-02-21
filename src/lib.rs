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

pub mod client;
pub mod category;
pub mod releases;
pub mod release;
pub mod series;
pub mod tags;
pub mod related_tags;
pub mod sources;
pub mod source;

mod error;