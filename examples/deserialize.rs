use anydate::serde::deserialize::anydate_utc;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Test {
    #[serde(deserialize_with = "anydate_utc")]
    dt: DateTime<Utc>,
}

fn main() {
    let dt: Test = serde_json::from_value(json!({"dt":"2021-11-14"})).unwrap();
    println!("{:?}", dt);
}
