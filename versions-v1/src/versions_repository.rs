use serde::de::Unexpected::Option;
use uuid::Uuid;
use crate::version::Version;
use crate::versions_read_write as VersionsReadWrite;

pub fn get_id() -> String {
    let uuid = Uuid::new_v4().to_string().replace("-", "");
    uuid[0..16].to_string()
}

pub fn insert_version(mut version: Version) {
    let mut versions = VersionsReadWrite::read().unwrap();

    let same_version = versions.iter()
        .find(|v| v.name == version.name && v.version == version.version);

    if same_version.is_some() {
        println!("Same version exists, aborting insert");
    } else {
        version.id = get_id();
        version.version_score = version.get_score();

        versions.push(version);

        VersionsReadWrite::write(&versions).unwrap();
    }
}

pub fn select_all_versions() -> Vec<Version> {
    VersionsReadWrite::read().unwrap()
}
