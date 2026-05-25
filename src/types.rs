use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Deserialize, Serialize};
use wincode::{SchemaWrite, SchemaRead};

#[derive(Debug, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize, SchemaWrite, SchemaRead)]
pub struct Person {
    pub name: String,
    pub age: u32,
}
