use bson::serde_helpers::uuid_1_as_binary;
use chrono::{DateTime, Utc};
use rust_tradier::account::Order;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Uniqueness {
    fn uniqueness(&self) -> String;
}

impl Uniqueness for Order {
    fn uniqueness(&self) -> String {
        format!("or|{}", self.id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wrapper<T> {
    // #[serde(serialize_with = "serialize_uuid", deserialize_with = "deserialize_uuid")]
    #[serde(with = "uuid_1_as_binary")]
    pub _id: Uuid,
    pub ts: DateTime<Utc>,
    pub uniqueness: String,
    pub obj: T,
}

pub trait Decorate {
    fn decorate(self) -> Wrapper<Self>
    where
        Self: Sized;
}

impl<T: Uniqueness> Decorate for T {
    fn decorate(self) -> Wrapper<Self> {
        let uuid = Uuid::now_v7();
        Wrapper {
            _id: uuid,
            ts: Utc::now(),
            uniqueness: self.uniqueness(),
            obj: self,
        }
    }
}

// fn serialize_uuid<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer {
//     serializer.serialize_bytes(uuid.as_bytes())
// }

// fn deserialize_uuid<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
// where
//     D: serde::Deserializer<'de> {
//     let bytes = <[u8; 16]>::deserialize(deserializer)?;
//     Ok(Uuid::from(bytes))
// }
