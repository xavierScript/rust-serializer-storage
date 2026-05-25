use rust_serializer_storage::{Storage, WincodeStorage, BorshSerializer, JsonSerializer, Person};

fn main() {
    let person = Person { name: "David".to_string(), age: 55 };

    let mut borsh_storage = Storage::new(BorshSerializer);
    borsh_storage.save(&person).unwrap();

    println!("Borsh bytes: {:?}", borsh_storage.data);
    println!("Borsh loaded:   {:?}", borsh_storage.load().unwrap());

    let mut json_storage = Storage::new(JsonSerializer);
    json_storage.save(&person).unwrap();

    println!("JSON bytes:    {:?}", json_storage.data);
    println!("JSON loaded:    {:?}", json_storage.load().unwrap());

    let mut wincode_storage = WincodeStorage::new();
    wincode_storage.save(&person).unwrap();

    println!("Wincode bytes:  {:?}", wincode_storage.data);
    println!("Wincode loaded: {:?}", wincode_storage.load().unwrap());
}
