use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Processor {
    name: String
}

#[allow(dead_code)] // Because get_name() is not used
impl Processor {
    pub fn new() -> Self {
        Processor {
            name: String::new()
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
