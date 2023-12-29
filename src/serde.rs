//! serde helper deserialize functions
use chrono::{DateTime, FixedOffset, Utc};
use core::fmt;
use serde::de;

struct AnydateVisitor;

impl<'de> de::Visitor<'de> for AnydateVisitor {
    type Value = DateTime<FixedOffset>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a formatted date and time string or a unix timestamp")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        crate::datetime::parse(value).map_err(E::custom)
    }
}

pub mod deserialize {
    //! deserialize helper functions
    //!
    //! ## Example
    //! ```rust
    //! use anydate::serde::deserialize::anydate_utc;
    //! use chrono::{DateTime, Utc};
    //! use serde::Deserialize;
    //! use serde_json::json;
    //!
    //! #[derive(Deserialize, Debug)]
    //! struct Test {
    //!     #[serde(deserialize_with = "anydate_utc")]
    //!     dt: DateTime<Utc>,
    //! }
    //!
    //! let dt: Test = serde_json::from_value(json!({"dt":"2021-11-14"})).unwrap();
    //! println!("{:?}", dt);
    //!
    //! ```
    use super::*;

    /// deserializes to a [`DateTime<FixedOffset>`]
    pub fn anydate<'de, D>(d: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(AnydateVisitor)
    }

    /// deserializes to a [`Option<DateTime<FixedOffset>>`]
    pub fn anydate_option<'de, D>(d: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_str(AnydateVisitor)
            .map_or_else(|_| None, Some))
    }

    /// deserializes to a [`DateTime<Utc>`]
    pub fn anydate_utc<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_str(AnydateVisitor)?.with_timezone(&Utc))
    }

    /// deserializes to a [`Option<DateTime<Utc>>`]
    pub fn anydate_utc_option<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_str(AnydateVisitor)
            .map_or_else(|_| None, |dt| Some(dt.with_timezone(&Utc))))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde::Deserialize;
        use serde_json::json;

        #[test]
        fn deserialize_any() -> Result<(), Box<dyn std::error::Error>> {
            #[derive(Deserialize)]
            struct Test {
                #[serde(deserialize_with = "anydate")]
                dt: DateTime<FixedOffset>,
            }
            for (input, expected) in [
                (json!({"dt":"2021-11-15T02:42:26Z"}), 1636944146000000000),
                (json!({"dt":"1636944446061"}), 1636944446061000000),
                (
                    json!({"dt":"Mon, 15 Nov 2021 02:49:37 +0000"}),
                    1636944577000000000,
                ),
            ] {
                let s: Test = serde_json::from_value(input)?;
                assert_eq!(s.dt.timestamp_nanos_opt().unwrap(), expected);
            }
            Ok(())
        }

        #[test]
        fn deserialize_any_option() -> Result<(), Box<dyn std::error::Error>> {
            #[derive(Deserialize)]
            struct Test {
                #[serde(deserialize_with = "anydate_option")]
                dt: Option<DateTime<FixedOffset>>,
            }
            for (input, expected) in [
                (
                    json!({"dt":"2021-11-15T02:42:26Z"}),
                    Some(1636944146000000000),
                ),
                (json!({"dt":"1636944446061"}), Some(1636944446061000000)),
                (
                    json!({"dt":"Mon, 15 Nov 2021 02:49:37 +0000"}),
                    Some(1636944577000000000),
                ),
                (json!({ "dt": null }), None),
                (json!({ "dt": "invalid junk" }), None),
            ] {
                let s: Test = serde_json::from_value(input)?;
                match expected {
                    Some(num) => {
                        assert_eq!(s.dt.unwrap().timestamp_nanos_opt().unwrap(), num);
                    }
                    None => {
                        assert_eq!(s.dt, None);
                    }
                };
            }
            Ok(())
        }

        #[test]
        fn deserialize_any_utc() -> Result<(), Box<dyn std::error::Error>> {
            #[derive(Deserialize)]
            struct Test {
                #[serde(deserialize_with = "anydate_utc")]
                dt: DateTime<Utc>,
            }
            for (input, expected) in [
                (json!({"dt":"2021-11-15T02:42:26Z"}), 1636944146000000000),
                (json!({"dt":"1636944446061"}), 1636944446061000000),
                (
                    json!({"dt":"Mon, 15 Nov 2021 02:49:37 +0000"}),
                    1636944577000000000,
                ),
            ] {
                let s: Test = serde_json::from_value(input)?;
                assert_eq!(s.dt.timestamp_nanos_opt().unwrap(), expected);
            }
            Ok(())
        }

        #[test]
        fn deserialize_any_utc_option() -> Result<(), Box<dyn std::error::Error>> {
            #[derive(Deserialize, Debug)]
            struct Test {
                #[serde(deserialize_with = "anydate_utc_option")]
                dt: Option<DateTime<Utc>>,
            }
            for (input, expected) in [
                (
                    json!({"dt":"2021-11-15T02:42:26Z"}),
                    Some(1636944146000000000),
                ),
                (json!({"dt":"1636944446061"}), Some(1636944446061000000)),
                (
                    json!({"dt":"Mon, 15 Nov 2021 02:49:37 +0000"}),
                    Some(1636944577000000000),
                ),
                (json!({ "dt": null }), None),
                (json!({ "dt": "invalid junk" }), None),
            ] {
                let s: Test = serde_json::from_value(input)?;
                match expected {
                    Some(num) => {
                        assert_eq!(s.dt.unwrap().timestamp_nanos_opt().unwrap(), num);
                    }
                    None => {
                        assert_eq!(s.dt, None);
                    }
                };
            }
            Ok(())
        }
    }
}
