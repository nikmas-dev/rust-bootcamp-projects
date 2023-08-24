use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs;
use std::path::Path;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

pub const REQUEST_FILE_PATH: &str = "3_ecosystem/3_6_serde/request.json";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "fail")]
    Fail,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Request {
    #[serde(flatten)]
    r#type: RequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<Stream>,
    gifts: Vec<Gift>,
    debug: Debug,
}

fn main() {
    let path = Path::new(REQUEST_FILE_PATH);

    let content = fs::read_to_string(path).unwrap();

    let request = serde_json::from_str::<Request>(&content).unwrap();
    println!("{:#?}", request);

    let yaml_output = serde_yaml::to_string(&request).unwrap();
    println!("{}", yaml_output);

    let toml_output = toml::to_string(&request).unwrap();
    println!("{}", toml_output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn test_request_deserialization() {
        let path = Path::new(REQUEST_FILE_PATH);

        let content = r#"{
              "type": "success",
              "stream": {
                "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
                "is_private": false,
                "settings": 45345,
                "shard_url": "https://n3.example.com/sapi",
                "public_tariff": {
                  "id": 1,
                  "price": 100,
                  "duration": "1h",
                  "description": "test public tariff"
                },
                "private_tariff": {
                  "client_price": 250,
                  "duration": "1m",
                  "description": "test private tariff"
                }
              },
              "gifts": [{
                "id": 1,
                "price": 2,
                "description": "Gift 1"
              }, {
                "id": 2,
                "price": 3,
                "description": "Gift 2"
              }],
              "debug": {
                "duration": "234ms",
                "at": "2019-06-28T08:35:46+00:00"
              }
            }
        "#;

        let request = serde_json::from_str::<Request>(&content).unwrap();
        let expected_request = Request {
            r#type: RequestType::Success,
            stream: Some(Stream {
                user_id: uuid!("8d234120-0bda-49b2-b7e0-fbd3912f6cbf"),
                is_private: false,
                settings: 45345,
                shard_url: Url::parse("https://n3.example.com/sapi").unwrap(),
                public_tariff: PublicTariff {
                    id: 1,
                    price: 100,
                    duration: "3600s".parse::<humantime::Duration>().unwrap().into(),
                    description: String::from("test public tariff"),
                },
                private_tariff: PrivateTariff {
                    client_price: 250,
                    duration: "60s".parse::<humantime::Duration>().unwrap().into(),
                    description: String::from("test private tariff"),
                },
            }),
            gifts: vec![
                Gift {
                    id: 1,
                    price: 2,
                    description: String::from("Gift 1"),
                },
                Gift {
                    id: 2,
                    price: 3,
                    description: String::from("Gift 2"),
                },
            ],
            debug: Debug {
                duration: "234ms".parse::<humantime::Duration>().unwrap().into(),
                at: DateTime::parse_from_rfc3339("2019-06-28T08:35:46+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
            },
        };

        assert_eq!(request, expected_request);
    }
}
