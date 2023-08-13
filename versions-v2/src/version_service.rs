use std::io::Error;
use uuid::Uuid;
use crate::read_write;
use crate::version::Version;

pub fn get_id() -> String {
    let uuid = Uuid::new_v4().to_string().replace("-", "");
    uuid[0..16].to_string()
}

pub fn capitalize_first_letter(version: &mut Version) {
    let mut current_name = version.name.to_lowercase();
    let mut first_character = current_name.chars().nth(0).unwrap();
    first_character.make_ascii_uppercase();
    version.name = first_character.to_string() + &current_name[1..current_name.len()]
}

pub fn insert(mut version: Version) -> Result<(), Error> {
    version.id = get_id();
    capitalize_first_letter(&mut version);
    version.version_score = version.get_score();

    let mut versions = read_write::read()?;
    versions.push(version);

    read_write::write(versions)
}

pub fn select_all() -> Result<Vec<Version>, Error> {
    read_write::read()
}
