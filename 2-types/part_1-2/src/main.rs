pub mod part_1;
mod part_2;

use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let s = include_str!("../../request.json");

    let deserialized: part_2::Request = serde_json::from_str(&s).unwrap();
    println!("\ndeserialized = {:?}", deserialized);

    let serialized: String = toml::to_string(&deserialized).unwrap();
    println!("\nserialized = {}", serialized);
}