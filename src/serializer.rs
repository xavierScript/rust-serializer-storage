use borsh::{BorshSerialize, BorshDeserialize, from_slice, to_vec};
use serde::{Deserialize, Serialize};

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

impl Serializer for WincodeSerializer {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, String>
    where
        T: BorshSerialize + Serialize,
    {
        bincode::serialize(value).map_err(|e| e.to_string())
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, String>
    where
        T: BorshDeserialize + for<'a> Deserialize<'a>,
    {
        bincode::deserialize(bytes).map_err(|e| e.to_string())
    }
}
