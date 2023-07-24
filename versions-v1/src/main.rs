mod version;
mod versions_read_write;
mod id_generator;
mod versions_repository;

use std::io::{Read, Write};
use crate::version::Version;

fn main() {

    let mut rust170 = Version::new();
    rust170.name = "Rust".to_string();
    rust170.version = "1.70.0".to_string();
    rust170.release_date = "2023-06-01".to_string();

    versions_repository::insert_version(rust170);

    let versions = versions_repository::select_all_versions();

    for version in versions {
        println!("{} {} {} {}", version.id, version.name, version.version, version.release_date);
    }

}
