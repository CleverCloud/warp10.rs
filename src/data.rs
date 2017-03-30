use time::Timespec;
use url::percent_encoding;

fn url_encode(input: &str) -> String {
    let mut s = String::new();
    s.extend(percent_encoding::utf8_percent_encode(input, percent_encoding::SIMPLE_ENCODE_SET));
    s
}

pub trait Warp10Serializable {
    fn warp10_serialize(&self) -> String;
}

pub type Int     = i32;
pub type Long    = i64;
pub type Double  = f64;
pub type Boolean = bool;

#[derive(Debug)]
pub enum Value {
    Int(Int),
    Long(Long),
    Double(Double),
    Boolean(Boolean),
    String(String),
}

impl Warp10Serializable for Value {
    fn warp10_serialize(&self) -> String {
        match *self {
            Value::Int(i)        => i.to_string(),
            Value::Long(l)       => l.to_string(),
            Value::Double(d)     => d.to_string(),
            Value::Boolean(b)    => b.to_string(),
            Value::String(ref s) => format!("'{}'", s)
        }
    }
}

#[derive(Debug)]
pub struct GeoValue {
    lat:  Double,
    lon:  Double,
    elev: Option<Long>,
}

impl GeoValue {
    pub fn new(lat: Double, lon: Double, elev: Option<Long>) -> GeoValue {
        GeoValue {
            lat:  lat,
            lon:  lon,
            elev: elev,
        }
    }
}

impl Warp10Serializable for GeoValue {
    fn warp10_serialize(&self) -> String {
        format!("{}:{}/{}",
                self.lat,
                self.lon,
                self.elev.map(|e| e.to_string()).unwrap_or("".to_string()))
    }
}

#[derive(Debug)]
pub struct Label<'a> {
    name:  &'a str,
    value: &'a str
}

impl<'a> Label<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Label<'a> {
        Label {
            name:  name,
            value: value
        }
    }
}

impl<'a> Warp10Serializable for Label<'a> {
    fn warp10_serialize(&self) -> String {
        format!("{}={}", url_encode(self.name), url_encode(self.value))
    }
}

#[derive(Debug)]
pub struct Data<'a> {
    date:   Timespec,
    geo:    Option<GeoValue>,
    name:   String,
    labels: Vec<Label<'a>>,
    value:  Value,
}

impl<'a> Data<'a> {
    pub fn new(date:   Timespec,
               geo:    Option<GeoValue>,
               name:   String,
               labels: Vec<Label<'a>>,
               value:  Value)
               -> Data<'a> {
        Data {
            date:   date,
            geo:    geo,
            name:   name,
            labels: labels,
            value:  value,
        }
    }
}

impl<'a> Warp10Serializable for Data<'a> {
    fn warp10_serialize(&self) -> String {
        let date_ms = self.date.sec * 1000000 + (self.date.nsec as Long) / 1000;
        let geo = match &self.geo {
            &None        => "/".to_string(),
            &Some(ref g) => g.warp10_serialize()
        };
        let labels = self.labels.iter().map(|l| l.warp10_serialize()).fold(String::new(), |acc, cur| {
            if acc.is_empty() {
                cur
            } else {
                (acc + ",") + &cur
            }
        });
        format!("{}/{} {}{{{}}} {}",
                date_ms,
                geo,
                url_encode(&self.name),
                labels,
                self.value.warp10_serialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use time::Timespec;

    #[test]
    fn serialize_int() {
        assert_eq!(Value::Int(42).warp10_serialize(), "42");
    }

    #[test]
    fn serialize_long() {
        assert_eq!(Value::Long(42).warp10_serialize(), "42");
    }

    #[test]
    fn serialize_double() {
        assert_eq!(Value::Double(42.66).warp10_serialize(), "42.66");
    }

    #[test]
    fn serialize_boolean() {
        assert_eq!(Value::Boolean(true).warp10_serialize(), "true");
        assert_eq!(Value::Boolean(false).warp10_serialize(), "false");
    }

    #[test]
    fn serialize_string() {
        assert_eq!(Value::String("foobar".to_string()).warp10_serialize(),
                   "'foobar'");
    }

    #[test]
    fn serialize_geo() {
        assert_eq!(GeoValue::new(42.66, 32.85, None).warp10_serialize(),
                   "42.66:32.85/");
        assert_eq!(GeoValue::new(42.66, 32.85, Some(10)).warp10_serialize(),
                   "42.66:32.85/10");
    }

    #[test]
    fn serialize_label() {
        assert_eq!(Label::new("name 1", "凄い value 2").warp10_serialize(), "name 1=%E5%87%84%E3%81%84 value 2");
    }

    #[test]
    fn serialize_data() {
        assert_eq!(Data::new(Timespec::new(25, 123456789),
                             None,
                             "original name".to_string(),
                             vec![
                                 Label::new("label1", "value1"),
                                 Label::new("label 2", "value 2")
                             ],
                             Value::String("foobar".to_string()))
                       .warp10_serialize(),
                   "25123456// original name{label1=value1,label 2=value 2} 'foobar'");
        assert_eq!(Data::new(Timespec::new(25, 123456789),
                             Some(GeoValue::new(42.66, 32.85, Some(10))),
                             "original name".to_string(),
                             vec![
                                 Label::new("label1", "value1"),
                                 Label::new("label 2", "value 2")
                             ],
                             Value::String("foobar".to_string()))
                       .warp10_serialize(),
                   "25123456/42.66:32.85/10 original name{label1=value1,label 2=value 2} 'foobar'");
    }
}
