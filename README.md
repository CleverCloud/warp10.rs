# warp10

Rust client for [Warp10 Geo/time series DB](http://www.warp10.io/).

[![Crates.io](https://img.shields.io/crates/v/warp10.svg)](https://crates.io/crates/warp10)
[![LICENSE](https://img.shields.io/github/license/CleverCloud/warp10.rs.svg)](COPYING)

## Features

- **Update** - write data points to Warp10
- **Exec** - execute WarpScript and get structured JSON results with execution metadata
- **Find** - discover time series matching a selector

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
warp10 = "3"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Examples

### Writing data points

```rust
use warp10::{Client, Data, GeoValue, Label, Value};

#[tokio::main]
async fn main() -> Result<(), warp10::Error> {
    let client = Client::builder()
        .url("http://localhost:8080")
        .write_token("my_write_token")
        .build()?;

    let writer = client.get_writer();
    let response = writer.post(vec![
        Data::new(
            time::OffsetDateTime::now_utc(),
            Some(GeoValue::new(42.66, 62.18, Some(10))),
            "test.data.name".to_string(),
            vec![
                Label::new("label1", "value1"),
                Label::new("label2", "value2"),
            ],
            Value::String("hello warp10".to_string()),
        ),
    ]).await?;

    println!("status: {:?}", response.status());
    Ok(())
}
```

### Executing WarpScript

```rust
use warp10::Client;

#[tokio::main]
async fn main() -> Result<(), warp10::Error> {
    let client = Client::builder().build()?;

    let response = client.exec("1 2 + 3 4 +").await?;

    println!("result: {}", response.body);       // [3, 7]
    println!("elapsed: {:?}", response.meta.elapsed);
    println!("ops: {:?}", response.meta.ops);
    println!("fetched: {:?}", response.meta.fetched);
    Ok(())
}
```

### Finding time series

```rust
use warp10::Client;

#[tokio::main]
async fn main() -> Result<(), warp10::Error> {
    let client = Client::builder()
        .read_token("my_read_token")
        .build()?;

    let response = client.find("~test.*{label1=value1}").await?;

    for series in response.series() {
        println!("{}", series);
    }
    Ok(())
}
```

### Custom reqwest client

```rust
use warp10::Client;

#[tokio::main]
async fn main() -> Result<(), warp10::Error> {
    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();

    let client = Client::builder()
        .url("http://warp10.example.com:8080")
        .write_token("my_token")
        .read_token("my_token")
        .http_client(http)
        .build()?;

    // use client...
    Ok(())
}
```

## MSRV

The minimum supported Rust version is **1.85.0**.
