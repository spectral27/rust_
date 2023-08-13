use std::fs::File;
use std::io::{Error, Read, Write};
use crate::version::Version;

const PATH: &str = "versions.json";

pub fn read() -> Result<Vec<Version>, Error> {
    let mut file_to_read = File::open(PATH)?;
    let mut versions_from_file = String::new();
    file_to_read.read_to_string(&mut versions_from_file)?;
    let versions: Vec<Version> = serde_json::from_str(&versions_from_file)?;
    Ok(versions)
}

pub fn write(versions: Vec<Version>) -> Result<(), Error> {
    let versions_json = serde_json::to_string_pretty(&versions)?;
    let mut file_to_write = File::create(PATH)?;
    file_to_write.write_all(versions_json.as_bytes())?;
    Ok(())
}
