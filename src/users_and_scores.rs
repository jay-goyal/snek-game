use serde_json::{from_str, to_value, to_writer, Value as JsonValue};
use std::collections::HashMap;
use std::fs::{write, File, OpenOptions};
use std::io::{Read, Seek};
use std::path::Path;

pub fn get_highscore(user: &String) -> Option<u64> {
    // Creating file if not exists else just open
    if !(Path::new("users.json").exists()) {
        File::create("users.json");
        write("users.json", "{}").expect("");
    }

    // Opening the file
    let mut file = OpenOptions::new().read(true).open("users.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    // Getting high score
    let json: JsonValue = from_str(&data).unwrap();
    let x = json[user].as_u64();
    match x {
        Some(score) => Some(score),
        None => Some(0),
    }
}

pub fn set_new_highscore(user: &String, highscore: u64) {
    // Opening file with write priviledges
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(false)
        .open("users.json")
        .unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    // Moving to start of file again
    file.rewind();

    let mut json_map: HashMap<&str, JsonValue> = from_str(&data).unwrap();

    json_map.insert(user, JsonValue::from(highscore));
    // let mut json: JsonValue = from_str(&data).unwrap();
    let json: JsonValue = to_value(json_map).unwrap();
    to_writer(file, &json);
}
