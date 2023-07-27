mod version;
mod versions_read_write;
mod id_generator;
mod versions_repository;

use std::io::{Read, Write};
use crate::version::Version;

fn main() {
    let v1 = Version::new_with_data("testversion".to_string(), "1.0.0".to_string(), "2023-07-01".to_string());

    versions_repository::update_version("d7826386ac454d71", v1);

    let versions = versions_repository::select_all_versions();

    for version in versions {
        println!("{:?}", version);
    }

}
