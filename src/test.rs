use serde::{Serialize, Deserialize};
use crate::prep::Uniqueness;

#[derive(Serialize,Deserialize,Debug)]
pub struct Test {
    pub id: u64,
    pub value: String
}

impl Uniqueness for Test {
    fn uniqueness(&self) -> String {
        self.id.to_string()
    }
}
