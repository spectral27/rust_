use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    id: String,
    name: String,
    version: String,
    version_score: i32,
    release_date: String
}

impl Version {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            version: String::new(),
            version_score: 0,
            release_date: String::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn version_score(&self) -> i32 {
        self.version_score
    }
    pub fn release_date(&self) -> &str {
        &self.release_date
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
    pub fn set_version_score(&mut self, version_score: i32) {
        self.version_score = version_score;
    }
    pub fn set_release_date(&mut self, release_date: String) {
        self.release_date = release_date;
    }

    pub fn get_score(&self) -> i32 {
        let version_array = self.version.split(".");
        let mut result = 0;
        let mut multiplier = 10_000;
        for element in version_array {
            let n = element.parse::<i32>().unwrap();
            result += n * multiplier;
            multiplier /= 100;
        }
        result
    }
}
