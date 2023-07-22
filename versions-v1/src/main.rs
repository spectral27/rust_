mod version;
mod file_read_write;

use std::fs::File;
use std::io::{Read, Write};
use version::Version;

fn main() {
    let mut v = Version::new();
    v.set_id("a0b1c2d3".to_string());
    v.set_name("Rust".to_string());
    v.set_version("1.71.0".to_string());

    println!("{}", v.get_score());

    let mut j = Version::new();
    j.set_name("Java".to_string());
    j.set_version("17.0.7".to_string());

    println!("{}", j.get_score());

    let mut versions_vector = Vec::new();
    versions_vector.push(v);
    versions_vector.push(j);

    let string_to_write = serde_json::to_string_pretty(&versions_vector).unwrap();
    println!("{}", string_to_write);

    let path: &str = "versions.json";

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("Error creating the file: {:?}", e)
    };

    match file.write_all(string_to_write.as_bytes()) {
        Ok(_) => None::<i32>,
        Err(e) => panic!("Error writing to file: {:?}", e)
    };

    // File::create(path).unwrap().write_all(string_to_write.as_bytes()).unwrap();

    // File::open("versions.json").unwrap().read_to_string(&mut text_from_file).unwrap();

    let versions_from_file = match file_read_write::read() {
        Ok(versions) => versions,
        Err(e) => panic!("ERROR: {:?}", e)
    };

    for version in versions_from_file {
        println!("{:?}", version);
    }

}
