use borsh::{BorshSerialize, BorshDeserialize, from_slice, to_vec};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

// use wincode::{serialize, deserialize};
// use serde_json::{from_slice, to_vec};

// ── Serializer trait and implementations ─────────────────────────────────────────
trait Serializer {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, String>
    where
        T: BorshSerialize + Serialize;

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, String>
    where
        T: BorshDeserialize + for<'a> Deserialize<'a>;
}


struct BorshSerializer;

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

struct JsonSerializer;

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

struct WincodeSerializer;

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
// ── Generic Storage container ────────────────────────────────────────────────

struct Storage<T, S>
where
    T: BorshSerialize + BorshDeserialize + Serialize + for<'a> Deserialize<'a>,
    S: Serializer,
{
    data: Option<Vec<u8>>,   
    serializer: S,           
    _phantom: PhantomData<T>,
}

impl<T, S> Storage<T, S>
where
    T: BorshSerialize + BorshDeserialize + Serialize + for<'a> Deserialize<'a>,
    S: Serializer,
{
    /// Create an empty storage backed by the given serializer.
    fn new(serializer: S) -> Self {
        Self {
            data: None,
            serializer,
            _phantom: PhantomData,
        }
    }

    /// Serialize `value` and keep the bytes internally.
    fn save(&mut self, value: &T) -> Result<(), String> {
        let bytes = self.serializer.to_bytes(value)?;
        self.data = Some(bytes);
        Ok(())
    }

    /// Deserialize and return the stored value.
    /// Returns an error if nothing has been saved yet.
    fn load(&self) -> Result<T, String> {
        match &self.data {
            Some(bytes) => self.serializer.from_bytes(bytes),
            None => Err("Storage is empty".to_string()),
        }
    }

    /// Returns true if bytes are currently stored.
    fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

// ── Test data type ───────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_person() -> Person {
        Person { name: "David".to_string(), age: 55 }
    }

    #[test]
    fn test_borsh_complete_flow() {
        let person = make_person();
        let mut storage = Storage::new(BorshSerializer);

        assert!(!storage.has_data());
        storage.save(&person).unwrap();
        assert!(storage.has_data());

        let loaded = storage.load().unwrap();
        assert_eq!(loaded, person);
    }

    #[test]
    fn test_json_complete_flow() {
        let person = make_person();
        let mut storage = Storage::new(JsonSerializer);
        storage.save(&person).unwrap();
        assert_eq!(storage.load().unwrap(), person);
    }

    #[test]
    fn test_wincode_complete_flow() {
        let person = make_person();
        let mut storage = Storage::new(WincodeSerializer);
        storage.save(&person).unwrap();
        assert_eq!(storage.load().unwrap(), person);
    }

    #[test]
    fn test_load_empty_returns_error() {
        let storage: Storage<Person, _> = Storage::new(BorshSerializer);
        assert!(storage.load().is_err());
    }
}

fn main() {
    let person = Person { name: "David".to_string(), age: 55 };

    // Borsh
    let mut borsh_storage = Storage::new(BorshSerializer);
    borsh_storage.save(&person).unwrap();

    println!("Borsh bytes: {:?}", borsh_storage.data);
    println!("Borsh loaded:   {:?}", borsh_storage.load().unwrap());

    // JSON
    let mut json_storage = Storage::new(JsonSerializer);
    json_storage.save(&person).unwrap();

    println!("JSON bytes:    {:?}", json_storage.data);
    println!("JSON loaded:    {:?}", json_storage.load().unwrap());

    // Wincode
    let mut wincode_storage = Storage::new(WincodeSerializer);
    wincode_storage.save(&person).unwrap();

    println!("Wincode bytes:  {:?}", wincode_storage.data);
    println!("Wincode loaded: {:?}", wincode_storage.load().unwrap());
}
