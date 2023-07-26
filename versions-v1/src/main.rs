mod version;
mod versions_read_write;
mod id_generator;
mod versions_repository;

use std::io::{Read, Write};
use crate::version::Version;

fn main() {

    let mut rust171 = Version::new_with_data("Rust".to_string(), "1.71.0".to_string(), "2023-07-13".to_string());

    versions_repository::insert_version(rust171);

    let versions = versions_repository::latest();

    for version in versions {
        println!("{} {} {} {}", version.id, version.name, version.version, version.release_date);
    }

}
