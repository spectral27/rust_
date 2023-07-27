use std::any::Any;
use uuid::Uuid;
use crate::version::Version;
use crate::versions_read_write as VersionsReadWrite;

pub fn capitalize_first_letter(s: &str) -> String {
    let mut name_to_edit = s.to_lowercase();
    name_to_edit.get_mut(0..1).unwrap().make_ascii_uppercase();
    name_to_edit
}

pub fn get_id() -> String {
    let uuid = Uuid::new_v4().to_string().replace("-", "");
    uuid[0..16].to_string()
}

pub fn insert_version(mut version: Version) {
    version.name = capitalize_first_letter(&version.name);

    let mut versions = VersionsReadWrite::read().unwrap();

    let same_version = versions.iter()
        .find(|v| v.name == version.name && v.version == version.version);

    if same_version.is_some() {
        println!("Same version exists, aborting insert");
    } else {
        version.id = get_id();
        version.version_score = version.get_score();

        versions.push(version);

        VersionsReadWrite::write(versions).unwrap();
    }
}

pub fn select_all_versions() -> Vec<Version> {
    VersionsReadWrite::read().unwrap()
}

pub fn latest() -> Vec<Version> {
    let versions = VersionsReadWrite::read().unwrap();

    let mut names: Vec<String> = versions.iter()
        .map(|version| version.name.clone())
        .collect();

    names.sort();
    names.dedup();

    let mut latest: Vec<Version> = Vec::new();

    for name in names {
        let mut versions_by_name: Vec<Version> = versions.iter()
            .filter(|version| version.name == name)
            .map(|version| version.clone())
            .collect();

        versions_by_name.sort_by(|a, b| a.version_score.cmp(&b.version_score).reverse());

        let latest_version: Version = versions_by_name.swap_remove(0);

        latest.push(latest_version);
    }

    latest
}

pub fn get_versions_by_name(name: &str) -> Vec<Version> {
    let name = capitalize_first_letter(name);

    let versions = VersionsReadWrite::read().unwrap();

    let mut versions_by_name: Vec<Version> = versions.iter()
        .filter(|v| v.name == name)
        .map(|v| v.clone())
        .collect();

    versions_by_name.sort_by(|a, b| a.version_score.cmp(&b.version_score).reverse());

    versions_by_name
}

pub fn update_version(id: &str, updated_version: Version) {
    let mut versions = select_all_versions();

    let index = versions.iter()
        .enumerate()
        .find(|(_, v)| v.id == id)
        .map(|(index, _)| index);

    if index.is_some() {
        let mut version_to_update = versions.remove(index.unwrap());

        if updated_version.name.len() > 0 {
            let name = capitalize_first_letter(&updated_version.name);
            version_to_update.name = name;
        }

        if updated_version.version.len() > 0 {
            version_to_update.version = updated_version.version;
            version_to_update.version_score = version_to_update.get_score();
        }

        if updated_version.release_date.len() > 0 {
            version_to_update.release_date = updated_version.release_date
        }

        versions.push(version_to_update);

        VersionsReadWrite::write(versions).unwrap();
    } else {
        println!("Version with id {} not present.", id);
    }
}
