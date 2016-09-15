use itertools::Itertools;
use time::Timespec;
use url::form_urlencoded;

pub trait Warp10Serializable {
    fn warp10_serialize(&self) -> String;
}

#[derive(Debug)]
pub enum Warp10Value {
    Int(i32),
    Long(i64),
    Double(f64),
    Boolean(bool),
    String(String),
}

impl Warp10Serializable for Warp10Value {
    fn warp10_serialize(&self) -> String {
        use Warp10Value::*;
        match *self {
            Int(i)        => i.to_string(),
            Long(l)       => l.to_string(),
            Double(d)     => d.to_string(),
            Boolean(b)    => b.to_string(),
            String(ref s) => format!("'{}'", s)
        }
    }
}

#[derive(Debug)]
pub struct Warp10GeoValue {
    lat: f64,
    lon: f64,
    alt: Option<f64>
}

impl Warp10GeoValue {
    pub fn new(lat: f64, lon: f64, alt: Option<f64>) -> Warp10GeoValue {
        Warp10GeoValue {
            lat: lat,
            lon: lon,
            alt: alt
        }
    }
}

impl Warp10Serializable for Warp10GeoValue {
    fn warp10_serialize(&self) -> String {
        format!("{}:{}/{}", self.lat, self.lon, self.alt.map(|a| a.to_string()).unwrap_or("".to_string()))
    }
}

#[derive(Debug)]
pub struct Warp10Data {
    date:   Timespec,
    geo:    Option<Warp10GeoValue>,
    name:   String,
    labels: Vec<(String, String)>,
    value:  Warp10Value
}

impl Warp10Data {
    pub fn new(date: Timespec, geo: Option<Warp10GeoValue>, name: String, labels: Vec<(String, String)>, value: Warp10Value) -> Warp10Data {
        Warp10Data {
            date:   date,
            geo:    geo,
            name:   name,
            labels: labels,
            value:  value
        }
    }
}

fn url_encode_pair(key: &str, value: &str) -> String {
    form_urlencoded::Serializer::new(String::new()).append_pair(key, value).finish()
}

fn url_encode(input: &str) -> String {
    let mut s = String::new();
    s.extend(form_urlencoded::byte_serialize(input.as_bytes()));
    s
}

impl Warp10Serializable for Warp10Data {
    fn warp10_serialize(&self) -> String {
        let date_ms = self.date.sec * 1000000 + (self.date.nsec as i64) / 1000;
        let geo = match &self.geo {
            &None        => "/".to_string(),
            &Some(ref g) => g.warp10_serialize()
        };
        let labels = self.labels.iter().map(|&(ref k, ref v)| url_encode_pair(k, v)).join(",");
        format!("{}/{} {}{{{}}} {}", date_ms, geo, url_encode(&self.name), labels, self.value.warp10_serialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use time::Timespec;

    #[test]
    fn serialize_int() {
        assert_eq!(Warp10Value::Int(42).warp10_serialize(), "42");
    }

    #[test]
    fn serialize_long() {
        assert_eq!(Warp10Value::Long(42).warp10_serialize(), "42");
    }

    #[test]
    fn serialize_double() {
        assert_eq!(Warp10Value::Double(42.66).warp10_serialize(), "42.66");
    }

    #[test]
    fn serialize_boolean() {
        assert_eq!(Warp10Value::Boolean(true).warp10_serialize(),  "true");
        assert_eq!(Warp10Value::Boolean(false).warp10_serialize(), "false");
    }

    #[test]
    fn serialize_string() {
        assert_eq!(Warp10Value::String("foobar".to_string()).warp10_serialize(), "'foobar'");
    }

    #[test]
    fn serialize_geo() {
        assert_eq!(Warp10GeoValue::new(42.66, 32.85, None).warp10_serialize(), "42.66:32.85/");
        assert_eq!(Warp10GeoValue::new(42.66, 32.85, Some(10.2)).warp10_serialize(), "42.66:32.85/10.2");
    }

    #[test]
    fn serialize_data() {
        assert_eq!(Warp10Data::new(Timespec::new(25, 123456789), None, "original name".to_string(), vec![("label1".to_string(), "value1".to_string()), ("label 2".to_string(), "value 2".to_string())], Warp10Value::String("foobar".to_string())).warp10_serialize(), "25123456// original+name{label1=value1,label+2=value+2} 'foobar'");
        assert_eq!(Warp10Data::new(Timespec::new(25, 123456789), Some(Warp10GeoValue::new(42.66, 32.85, Some(10.2))), "original name".to_string(), vec![("label1".to_string(), "value1".to_string()), ("label 2".to_string(), "value 2".to_string())], Warp10Value::String("foobar".to_string())).warp10_serialize(), "25123456/42.66:32.85/10.2 original+name{label1=value1,label+2=value+2} 'foobar'");
    }
}
