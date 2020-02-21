# fred-rs

 **fred-rs is a simple interface for accessing the Federal Reserve Bank of St. Louis's FRED API.**

 ---
 ```toml
 [dependencies]
 fred_rs = "0.1.0"
 ```

 FRED is a large online database of economic data hosted by the Federal Reserve Bank of St. Louis.  The website currently hosts approximately "672,000 US and international time series from 89 sources" including the the graph of unemployment rate shown below.

![Unemployment Graph](README/fredgraph.png)

Access to the raw data is available through the FRED API.  fred-rs is an intermediate layer between the HTTP client and the user application.  Requests to the FRED API are made through structured calls to the fred_rs FredClient and data is returned as usable data objects (structs).

## Usage
Below is an example of the general usage for accessing an observation or data series.

```rust
use fred_rs::client::FredClient;
use fred_rs::series::observation::{Builder, Units, Frequency, Response};

// Create the client object
let mut c = match FredClient::new() {
    Ok(c) => c,
    Err(msg) => {
        println!("{}", msg);
        return
    },
};

// Create the argument builder
let mut builder = Builder::new();

// Set the arguments for the builder
builder
    .observation_start("2000-01-01")
    .units(Units::PCH)
    .frequency(Frequency::M);

// Make the request and pass in the builder to apply the arguments
let resp: Response = match c.series_observation("GNPCA", Some(builder)) {
    Ok(resp) => resp,
    Err(msg) => {
        println!("{}", msg);
        return
    },
};
```

## API Key
Developers need to request an API Key in order to access FRED.  This can be done at [https://research.stlouisfed.org/docs/api/api_key.html](https://research.stlouisfed.org/docs/api/api_key.html).

fred-rs looks for the `FRED_API_KEY` environment variable by default.  This can be set using
```bash
export FRED_API_KEY=abcdefghijklmnopqrstuvwxyz123456
```

Alternatively, the `FredClient.with_key()` function allows the key to be set from a string reference.
```rust
use fred_rs::client::FredClient;

let mut client = match FredClient::new() {
    Ok(c) => c,
    Err(msg) => {
        println!("{}", msg);
        return
    },
};

client.with_key("abcdefghijklmnopqrstuvwxyz123456");
```
