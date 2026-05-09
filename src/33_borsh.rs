//! Borsh
// Borsh (Binary Object Representation Serializer for Hashing) is a deterministic, binary serialization format often used in Rust (and other languages) to encode and decode data in a consistent, unambiguous way          It was originally developed by the [NEAR Protocol]team for use in smart contracts, but you can use it in any Rust project that needs a fast, predictable serialization layer.

// For using Borsh we need dependecies
// Borsh     // note : in Cargo.toml add features = ["derive"] | so to use macros  


// ! NOTE : borsh do not store keys only the value and during deserialiazation it try to map the keys from the struct that we pass in it 
// here for example : Person

// ============================================================

// macro used
// #[derive(BorshSerialize, BorshDeserialize, Debug)]

// ============================================================




use borsh::{BorshSerialize, BorshDeserialize};
// macro : Serialize, Deserialize
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Person {
    first_name: String,     
    age: u32, 
    v: Vec<u32>
}

// experiment
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct DummyPerson {
    new_first_name: String,     
    new_age: u32, 
    new_v: Vec<u32>
}


fn main(){

    let person = Person {
        first_name: String::from("tushar"),
        age: 30,
        v: vec![1, 2, 3]
    };


    // Serialization : struct -> Borsh
    // ! NOTE : borsh do not store keys only the value and during deserialiazation it try to map the keys from the struct that we pass in it 
    let mut buffer: Vec<u8> = Vec::new();
    
    person.serialize(&mut buffer).unwrap();

    println!(" Borsh :\n {:?}",buffer);

    // deserialization : Borsh formates -> struct
    // Person struct is taken for refrance for keys
    let struct_deserialized  = Person::try_from_slice(&mut buffer).unwrap();

    println!(" rust struct :\n {:?}", struct_deserialized);


    // Experiment : it can be deserialized with the keys with same types but the key names can be different
    let exp  = DummyPerson::try_from_slice(&mut buffer).unwrap();
    println!(" rust struct :\n {:?}", exp);
    
}