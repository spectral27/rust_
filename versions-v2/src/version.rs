use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub id: String,
    pub name: String,
    pub version: String,
    pub version_score: i32,
    pub release_date: String
}

impl Version {
    pub fn new() -> Version {
        Version {
            id: String::new(),
            name: String::new(),
            version: String::new(),
            version_score: 0,
            release_date: String::new(),
        }
    }

    pub fn new_with_data(name: String, version: String, release_date: String) -> Version {
        Version {
            id: String::new(),
            name,
            version,
            version_score: 0,
            release_date
        }
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
