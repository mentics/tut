use chrono::{DateTime, Utc};
use rust_tradier::account::Order;
use uuid7::Uuid;
use serde::{Serialize, Deserialize};

pub trait NativeId {
    fn native_id(&self) -> String;
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Test {
    pub id: u64,
    pub value: String
}

impl NativeId for Test {
    fn native_id(&self) -> String {
        self.id.to_string()
    }
}

impl NativeId for Order {
    fn native_id(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Wrapper<T> {
    #[serde(serialize_with = "serialize_uuid", deserialize_with = "deserialize_uuid")]
    pub _id: Uuid,
    pub ts: DateTime<Utc>,
    pub native_id: String,
    pub obj: T,
}

pub trait Decorate {
    fn decorate(self) -> Wrapper<Self> where Self: Sized;
}

impl<T: NativeId> Decorate for T {
    fn decorate(self) -> Wrapper<Self> {
        let uuid = uuid7::uuid7();
        Wrapper { _id: uuid, ts: Utc::now(), native_id: self.native_id(), obj: self }
    }
}

fn serialize_uuid<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
    serializer.serialize_bytes(uuid.as_bytes())
}

fn deserialize_uuid<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: serde::Deserializer<'de> {
    let bytes = <[u8; 16]>::deserialize(deserializer)?;
    Ok(Uuid::from(bytes))
}
