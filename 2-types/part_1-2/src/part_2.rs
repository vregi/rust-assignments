use serde::{Serialize, Deserialize};
use url::Url;
use uuid::Uuid;
use std::time::Duration;
use chrono::DateTime;
use chrono::Utc;
use duration_str::deserialize_duration;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum RequestType {
  Success,
  Fail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff:PrivateTariff,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(deserialize_with = "deserialize_duration")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateTariff {
    client_price: u32,
    #[serde(deserialize_with = "deserialize_duration")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Debug {
    #[serde(deserialize_with = "deserialize_duration")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_request_deserialize_sample_success() {
        let data = r#"
        {
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

        let v: Request = from_str(data).unwrap();
        assert_eq!(v.r#type, "success");
        assert_eq!(v.stream.user_id, "8d234120-0bda-49b2-b7e0-fbd3912f6cbf");
        assert!(!v.stream.is_private);
        assert_eq!(v.stream.settings, 45345);
        assert_eq!(v.stream.shard_url, "https://n3.example.com/sapi");
        assert_eq!(v.stream.public_tariff.id, 1);
        assert_eq!(v.stream.public_tariff.price, 100);
        assert_eq!(v.stream.public_tariff.duration, "1h");
        assert_eq!(v.stream.public_tariff.description, "test public tariff");
        assert_eq!(v.stream.private_tariff.client_price, 250);
        assert_eq!(v.stream.private_tariff.duration, "1m");
        assert_eq!(v.stream.private_tariff.description, "test private tariff");
        assert_eq!(v.gifts.len(), 2);
        assert_eq!(v.gifts[0].id, 1);
        assert_eq!(v.gifts[0].price, 2);
        assert_eq!(v.gifts[0].description, "Gift 1");
        assert_eq!(v.gifts[1].id, 2);
        assert_eq!(v.gifts[1].price, 3);
        assert_eq!(v.gifts[1].description, "Gift 2");
        assert_eq!(v.debug.duration, "234ms");
        assert_eq!(v.debug.at, "2019-06-28T08:35:46+00:00");
    }
}
