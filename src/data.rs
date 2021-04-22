use time::OffsetDateTime;

fn url_encode(input: &str) -> String {
    let mut s = String::new();
    s.extend(percent_encoding::utf8_percent_encode(
        input,
        percent_encoding::NON_ALPHANUMERIC,
    ));
    s
}

pub trait Warp10Serializable {
    fn warp10_serialize(&self) -> String;
}

pub type Int = i32;
pub type Long = i64;
pub type Double = f64;
pub type Boolean = bool;

#[derive(Debug, Clone, PartialEq)]
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
            Value::Int(i) => i.to_string(),
            Value::Long(l) => l.to_string(),
            Value::Double(d) => d.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::String(ref s) => format!("'{}'", s),
        }
    }
}

impl From<Int> for Value {
    fn from(i: Int) -> Self {
        Self::Int(i)
    }
}

impl From<Long> for Value {
    fn from(l: Long) -> Self {
        Self::Long(l)
    }
}

impl From<Double> for Value {
    fn from(d: Double) -> Self {
        Self::Double(d)
    }
}

impl From<Boolean> for Value {
    fn from(b: Boolean) -> Self {
        Self::Boolean(b)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeoValue {
    lat: Double,
    lon: Double,
    elev: Option<Long>,
}

impl GeoValue {
    pub fn new(lat: Double, lon: Double, elev: Option<Long>) -> GeoValue {
        GeoValue {
            lat,
            lon,
            elev,
        }
    }
}

impl Warp10Serializable for GeoValue {
    fn warp10_serialize(&self) -> String {
        format!(
            "{}:{}/{}",
            self.lat,
            self.lon,
            self.elev
                .map(|e| e.to_string())
                .unwrap_or_else(|| "".to_string())
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    name: String,
    value: String,
}

impl Label {
    pub fn new(name: &str, value: &str) -> Label {
        Label {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl Warp10Serializable for Label {
    fn warp10_serialize(&self) -> String {
        format!("{}={}", url_encode(&self.name), url_encode(&self.value))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    date: Option<OffsetDateTime>,
    geo: Option<GeoValue>,
    name: String,
    labels: Vec<Label>,
    value: Value,
}

impl Data {
    pub fn new(
        date: OffsetDateTime,
        geo: Option<GeoValue>,
        name: String,
        labels: Vec<Label>,
        value: Value,
    ) -> Data {
        Data {
            date: Some(date),
            geo,
            name,
            labels,
            value,
        }
    }

    pub fn new_without_time(
        geo: Option<GeoValue>,
        name: String,
        labels: Vec<Label>,
        value: Value,
    ) -> Data {
        Data {
            date: None,
            geo,
            name,
            labels,
            value,
        }
    }
}

impl Warp10Serializable for Data {
    fn warp10_serialize(&self) -> String {
        let geo = self
            .geo
            .as_ref()
            .map(Warp10Serializable::warp10_serialize)
            .unwrap_or_else(|| "/".to_string());
        let labels =
            self.labels
                .iter()
                .map(|l| l.warp10_serialize())
                .fold(String::new(), |acc, cur| {
                    if acc.is_empty() {
                        cur
                    } else {
                        (acc + ",") + &cur
                    }
                });

        match self.date {
            Some(date) => {
                let date_ms = date.unix_timestamp() * 1000000 + (date.microsecond() as Long);
                format!(
                    "{}/{} {}{{{}}} {}",
                    date_ms,
                    geo,
                    url_encode(&self.name),
                    labels,
                    self.value.warp10_serialize()
                )
            }
            None => {
                // In this case the warp10 instance will put the same timestamp for the data point has
                // the ingestion one.
                format!(
                    "/{} {}{{{}}} {}",
                    geo,
                    url_encode(&self.name),
                    labels,
                    self.value.warp10_serialize()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use time::{Duration, OffsetDateTime};

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
        assert_eq!(
            Value::String("foobar".to_string()).warp10_serialize(),
            "'foobar'"
        );
    }

    #[test]
    fn serialize_geo() {
        assert_eq!(
            GeoValue::new(42.66, 32.85, None).warp10_serialize(),
            "42.66:32.85/"
        );
        assert_eq!(
            GeoValue::new(42.66, 32.85, Some(10)).warp10_serialize(),
            "42.66:32.85/10"
        );
    }

    #[test]
    fn serialize_label() {
        assert_eq!(
            Label::new("name 1", "凄い value 2").warp10_serialize(),
            "name%201=%E5%87%84%E3%81%84%20value%202"
        );
    }

    #[test]
    fn serialize_data() {
        assert_eq!(
            Data::new(
                OffsetDateTime::unix_epoch() + Duration::new(25, 123456789),
                None,
                "original name".to_string(),
                vec![
                    Label::new("label1", "value1"),
                    Label::new("label 2", "value 2"),
                ],
                Value::String("foobar".to_string())
            )
            .warp10_serialize(),
            "25123456// original%20name{label1=value1,label%202=value%202} 'foobar'"
        );
        assert_eq!(
            Data::new(
                OffsetDateTime::unix_epoch() + Duration::new(25, 123456789),
                Some(GeoValue::new(42.66, 32.85, Some(10))),
                "original name".to_string(),
                vec![
                    Label::new("label1", "value1"),
                    Label::new("label 2", "value 2"),
                ],
                Value::String("foobar".to_string())
            )
            .warp10_serialize(),
            "25123456/42.66:32.85/10 original%20name{label1=value1,label%202=value%202} 'foobar'"
        );
        assert_eq!(
            Data::new_without_time(
                Some(GeoValue::new(42.66, 32.85, Some(10))),
                "original name".to_string(),
                vec![
                    Label::new("label1", "value1"),
                    Label::new("label 2", "value 2"),
                ],
                Value::String("foobar".to_string())
            )
            .warp10_serialize(),
            "/42.66:32.85/10 original%20name{label1=value1,label%202=value%202} 'foobar'"
        );
    }
}
