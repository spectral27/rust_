mod version;
mod file_read_write;
mod id_generator;

use std::fs::File;
use std::io::{Read, Write};
use version::Version;

fn main() {
    // let mut v = Version::new();
    // v.id = id_generator::get_id();
    // v.name = "Rust".to_string();
    // v.version = "1.71.0".to_string();
    // v.version_score = v.get_score();
    // v.release_date = "2023-07-13".to_string();
    //
    // let mut versions_vector = Vec::new();
    // versions_vector.push(v);
    //
    // match file_read_write::write(&versions_vector) {
    //     Ok(_) => (),
    //     Err(e) => panic!("EXPLOSION! {:?}", e)
    // }
    //
    // let string_to_write = serde_json::to_string_pretty(&versions_vector).unwrap();
    // File::create(path).unwrap().write_all(string_to_write.as_bytes()).unwrap();
    //
    // File::open("versions.json").unwrap().read_to_string(&mut text_from_file).unwrap();

    let versions_from_file = match file_read_write::read() {
        Ok(versions) => versions,
        Err(e) => panic!("ERROR: {:?}", e)
    };

    for version in versions_from_file {
        println!("{}", version.id);
        println!("{:?}", version);
    }

}
