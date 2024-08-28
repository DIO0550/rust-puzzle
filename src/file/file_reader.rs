use std::io::Error;

use serde_json::{from_str, json, to_string, Value};

pub struct FileReader;

pub trait FileReaderTrait {
    fn save_data(file_name: &str, value: Value);
    fn load_data(file_name: &str) -> Option<Value>;
}

impl FileReaderTrait for FileReader {
    fn save_data(file_name: &str, value: Value) {
        std::fs::write(file_name, to_string(&value).unwrap()).unwrap();
    }

    fn load_data(file_name: &str) -> Option<Value> {
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
