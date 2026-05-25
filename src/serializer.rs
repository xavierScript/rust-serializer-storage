use borsh::{BorshSerialize, BorshDeserialize, from_slice, to_vec};
use serde::{Deserialize, Serialize};
use wincode::{SchemaWrite, SchemaRead, config::DefaultConfig};

pub trait Serializer {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, String>
    where
        T: BorshSerialize + Serialize;

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, String>
    where
        T: BorshDeserialize + for<'a> Deserialize<'a>;
}

pub struct BorshSerializer;

impl Serializer for BorshSerializer {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, String>
    where
        T: BorshSerialize + Serialize,
    {
        to_vec(value).map_err(|e| e.to_string())
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, String>
    where
        T: BorshDeserialize + for<'a> Deserialize<'a>,
    {
        from_slice(bytes).map_err(|e| e.to_string())
    }
}

pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, String>
    where
        T: BorshSerialize + Serialize,
    {
        serde_json::to_vec(value).map_err(|e| e.to_string())
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, String>
    where
        T: BorshDeserialize + for<'a> Deserialize<'a>,
    {
        serde_json::from_slice(bytes).map_err(|e| e.to_string())
    }
}

pub struct WincodeSerializer;

impl WincodeSerializer {
    pub fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, String>
    where
        T: SchemaWrite<DefaultConfig, Src = T> + ?Sized,
    {
        wincode::serialize(value).map_err(|e| e.to_string())
    }

    pub fn from_bytes<'de, T>(&self, bytes: &'de [u8]) -> Result<T, String>
    where
        T: SchemaRead<'de, DefaultConfig, Dst = T>,
    {
        wincode::deserialize(bytes).map_err(|e| e.to_string())
    }
}
