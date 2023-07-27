mod version;
mod versions_read_write;
mod id_generator;
mod versions_repository;

use std::io::{Read, Write};
use crate::version::Version;

fn main() {
    let versions = versions_repository::get_versions_by_name(&mut String::from("rust"));

    for version in versions {
        println!("{} {} {} {}", version.id, version.name, version.version, version.release_date);
    }

}
