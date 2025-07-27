use serde_json::{from_str, to_string, Value};
use std::fs::File;
use std::io::prelude::*;

pub struct JsonFile;
impl JsonFile {
    pub fn save(file_name: &str, value: Value) {
        let file = File::create(file_name).expect("Unable to create file");
        serde_json::to_writer(file, &value).expect("Unable to write JSON to file");
    }

    pub fn load(file_name: &str) -> Option<Value> {
        let read_result = std::fs::read_to_string(file_name);

        match read_result {
            Ok(json_str) => {
                let Ok(result) = from_str(&json_str) else {
                    return None;
                };

                return Some(result);
            }
            Err(_) => return None,
        }
    }
}
