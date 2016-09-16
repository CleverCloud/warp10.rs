# This is a Rust client for [Warp10 Geo/time series DB](http://www.warp10.io/)

[![Build Status](https://travis-ci.org/CleverCloud/warp10.rs.svg?branch=master)](https://travis-ci.org/CleverCloud/warp10.rs)

## Features

At the moment, we support writing to warp10.

Reading support should come at some point.

## Example

```
extern crate hyper;
extern crate time;
extern crate warp10;

fn warp10_post() -> std::result::Result<hyper::client::Response, warp10::Error> {
    let client = try!(warp10::Client::new("http://localhost:8080/"));
    let writer = client.get_writer("my_write_token".to_string());
    let res    = try!(writer.post(vec![
        warp10::Data::new(
            time::now_utc().to_timespec(),
            Some(warp10::GeoValue::new(42.66, 62.18, Some(10))),
            "test data name 2".to_string(),
            vec![
                ("label 1 name".to_string(), "label 1 value".to_string()),
                ("label 2 name".to_string(), "label 2 value".to_string())
            ],
            warp10::Value::String("Test warp10 awesome value".to_string())
        )
    ]));
    Ok(res)
}

fn main() {
    println!("{:?}", warp10_post());
}
```
