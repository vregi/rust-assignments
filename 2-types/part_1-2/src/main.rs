pub mod part_1;
mod part_2;

use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("/home/vregi/Desktop/rust-assignments/2-types/request.json");
    let display = path.display();

   let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }

    let deserialized: part_2::Request = serde_json::from_str(&s).unwrap();
    println!("\ndeserialized = {:?}", deserialized);

    let serialized: String = toml::to_string(&deserialized).unwrap();
    println!("\nserialized = {}", serialized);
}