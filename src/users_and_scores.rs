use home::home_dir;
use serde_json::{from_str, to_value, to_writer, Value as JsonValue};
use std::collections::HashMap;
use std::env::current_exe;
use std::fs::{write, File, OpenOptions};
use std::io::{Read, Seek};

pub fn get_highscore(player: &String) -> Option<u64> {
    if player == "" {
        return Some(0);
    }

    let file = home_dir();
    let mut file = match file {
        Some(file) => file,
        None => {
            let mut x = current_exe().unwrap();
            x.pop();
            x
        }
    };

    file.push(".users.json");

    // Creating file if not exists else just open
    if !(file.exists()) {
        File::create(&file).expect("");
        write(&file, "{}").expect("");
    }

    // Opening the file
    let mut file = OpenOptions::new().read(true).open(file).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    // Getting high score
    let json: JsonValue = from_str(&data).unwrap();
    let x = json[player].as_u64();
    match x {
        Some(score) => Some(score),
        None => Some(0),
    }
}

pub fn set_new_highscore(player: &String, highscore: u64) {
    let file = home_dir();
    let mut file = match file {
        Some(file) => file,
        None => {
            let mut x = current_exe().unwrap();
            x.pop();
            x
        }
    };
    file.push(".users.json");

    // Opening file with write priviledges
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(false)
        .open(file)
        .unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    // Moving to start of file again
    file.rewind().expect("");

    let mut json_map: HashMap<&str, JsonValue> = from_str(&data).unwrap();

    json_map.insert(player, JsonValue::from(highscore));
    // let mut json: JsonValue = from_str(&data).unwrap();
    let json: JsonValue = to_value(json_map).unwrap();
    to_writer(file, &json).expect("");
}
