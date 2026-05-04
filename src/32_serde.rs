//! Serde
//Serde is a popular serialization and deserialization framework in Rust. It provides a way to convert Rust data structures into different formats (serialization) and vice versa (deserialization). The most common use cases involve working with formats like JSON, YAML, TOML, and others.

// rust struct => other formates (json, yaml, toml, etc)
// and 
// other formates => rust struct


// For using serde we need dependecies
// 1. serde           // note : in Cargo.toml add features = ["derive"] | so to use macros  
// 2. serde_derive   e.g for json : serde_json   toml : toml

// ============================================================

// macro used
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all="camelCase")]  
// #[serde(rename="apka_nam")]

// ============================================================



use serde::{Serialize, Deserialize};

// macro : Serialize, Deserialize
#[derive(Serialize, Deserialize, Debug)]  // debug :  {:?} print
#[serde(rename_all="camelCase")]  // macro converst the bellow fields to camelCase
struct Person {
    #[serde(rename="apka_nam")]   // macro rename the bellow field
    first_name: String,     // apka_nam
    second_name: String,    // secondName
    age: u32, 
}


fn main(){

    let person = Person {
        first_name: String::from("tushar"),
        second_name: "chauhan".to_string(),
        age: 30,
    };


    // Serialization : struct -> other formates 
    let json_string = serde_json::to_string(&person).unwrap();

    let toml_string = toml::to_string(&person).unwrap();

    println!(" json string formate :\n {}",json_string);
    println!(" toml string formate :\n {}",toml_string);


    // deserialization : other formates -> struct
    
    // let struct_json: Result<Person, serde_json::Error> = serde_json::from_str(&json_string);
    let struct_json = serde_json::from_str::<Person>(&json_string);
    
    let struct_toml: Result<Person, toml::de::Error> = toml::from_str(&toml_string);
    
    println!("{:?}", struct_json);
    println!("{:?}", struct_toml);

    
}