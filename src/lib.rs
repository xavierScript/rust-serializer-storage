pub mod serializer;
pub mod storage;
pub mod types;

pub use serializer::{Serializer, BorshSerializer, JsonSerializer, WincodeSerializer};
pub use storage::Storage;
pub use types::Person;

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
