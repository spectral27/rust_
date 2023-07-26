use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use serde::de::Unexpected::Option;
use crate::version::Version;

const PATH: &str = "versions.json";

pub fn read() -> Result<Vec<Version>, Box<dyn Error>> {
    // let mut text_from_file = String::new();
    // File::open("versions_.json").unwrap().read_to_string(&mut text_from_file).unwrap();

    if File::open(PATH).is_err() {
        File::create(PATH).unwrap();
    }

    let mut file_to_read = File::open(PATH).unwrap();

    let mut text_from_file = String::new();

    file_to_read.read_to_string(&mut text_from_file).unwrap();

    let versions_from_file = match serde_json::from_str(&text_from_file) {
        Ok(versions) => versions,
        Err(_) => Vec::new()
    };

    Ok(versions_from_file)
}

pub fn write(versions: &Vec<Version>) -> Result<(), Box<dyn Error>> {
    // let string_to_write = serde_json::to_string_pretty(&versions_vector).unwrap();
    // File::create(path).unwrap().write_all(string_to_write.as_bytes()).unwrap();

    let text_to_write = match serde_json::to_string_pretty(versions) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("EXPLOSION parsing vector to string: {:?}", e);
            return Err(e.into())
        }
    };

    let mut file_to_write = match File::create(PATH) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("EXPLOSION creating file: {:?}", e);
            return Err(e.into())
        }
    };

    match file_to_write.write_all(text_to_write.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("EXPLOSION writing to file: {:?}", e);
            return Err(e.into())
        }
    };

    Ok(())
}
