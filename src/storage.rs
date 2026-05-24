use crate::serializer::Serializer;
use borsh::{BorshSerialize, BorshDeserialize};
use serde::Deserialize;
use std::marker::PhantomData;

pub struct Storage<T, S>
where
    T: BorshSerialize + BorshDeserialize + serde::Serialize + for<'a> Deserialize<'a>,
    S: Serializer,
{
    pub data: Option<Vec<u8>>,
    pub serializer: S,
    _phantom: PhantomData<T>,
}

impl<T, S> Storage<T, S>
where
    T: BorshSerialize + BorshDeserialize + serde::Serialize + for<'a> Deserialize<'a>,
    S: Serializer,
{
    pub fn new(serializer: S) -> Self {
        Self {
            data: None,
            serializer,
            _phantom: PhantomData,
        }
    }

    pub fn save(&mut self, value: &T) -> Result<(), String> {
        let bytes = self.serializer.to_bytes(value)?;
        self.data = Some(bytes);
        Ok(())
    }

    pub fn load(&self) -> Result<T, String> {
        match &self.data {
            Some(bytes) => self.serializer.from_bytes(bytes),
            None => Err("Storage is empty".to_string()),
        }
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}
