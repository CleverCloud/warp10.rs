# This is a Rust client for [Warp10 Geo/time series DB](http://www.warp10.io/)

[![Crates.io](https://img.shields.io/crates/v/warp10.svg)](https://crates.io/crates/warp10)
[![Build Status](https://travis-ci.org/CleverCloud/warp10.rs.svg?branch=master)](https://travis-ci.org/CleverCloud/warp10.rs)
[![Build status](https://ci.appveyor.com/api/projects/status/a07ji027cs3t7jqu?svg=true)](https://ci.appveyor.com/project/Keruspe/warp10-rs)
[![Coverage Status](https://coveralls.io/repos/github/CleverCloud/warp10.rs/badge.svg?branch=master)](https://coveralls.io/github/CleverCloud/warp10.rs?branch=master)
[![LICENSE](https://img.shields.io/github/license/CleverCloud/warp10.rs.svg)](COPYING)

## Features

At the moment, we support writing to warp10.

Reading support should come at some point.

## Example

```rust
extern crate hyper;
extern crate time;
extern crate warp10;

fn warp10_post() -> std::result::Result<warp10::Response, warp10::Error> {
    let client = warp10::Client::new("http://localhost:8080/")?;
    let writer = client.get_writer("my_write_token".to_string());
    let res    = writer.post(vec![
        warp10::Data::new(
            time::now_utc().to_timespec(),
            Some(warp10::GeoValue::new(42.66, 62.18, Some(10))),
            "test data name 2".to_string(),
            vec![
                warp10::Label::new("label 1 name", "label 1 value"),
                warp10::Label::new("label 2 name", "label 2 value")
            ],
            warp10::Value::String("Test warp10 awesome value".to_string())
        )
    ])?;
    Ok(res)
}

fn main() {
    println!("{:?}", warp10_post());
}
```
