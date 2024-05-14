extern crate x11_clipboard;
extern crate serde;
extern crate serde_json;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use x11_clipboard::Clipboard;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct ClipboardItem {
    id: usize,
    content: String,
}

fn main() {
    let clipboard = Clipboard::new().unwrap();
    let mut id_counter = load_id_counter();

    loop {
        let val = clipboard
            .load_wait(
                clipboard.setter.atoms.clipboard,
                clipboard.setter.atoms.string,
                clipboard.setter.atoms.property,
            )
            .unwrap();

        let val = String::from_utf8(val).unwrap();
        if !val.is_empty() && !val.trim().is_empty() {
            let clipboard_item = ClipboardItem {
                id: id_counter,
                content: val.clone(),
            };

            append_to_json(&clipboard_item).expect("Failed to append to JSON");
            println!("Copied content appended to JSON: {:?}", clipboard_item);

            id_counter += 1;
            save_id_counter(id_counter);
        }
    }
}

fn load_id_counter() -> usize {
    let path = "id_counter.txt";
    if Path::new(path).exists() {
        let mut file = File::open(path).expect("Failed to open id_counter.txt");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read id_counter.txt");
        contents.trim().parse().expect("Failed to parse id_counter.txt")
    } else {
        0
    }
}

fn save_id_counter(id_counter: usize) {
    let path = "id_counter.txt";
    let mut file = File::create(path).expect("Failed to create id_counter.txt");
    write!(file, "{}", id_counter).expect("Failed to write id_counter.txt");
}

fn append_to_json(clipboard_item: &ClipboardItem) -> io::Result<()> {
    let path = "clipboard.json";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let mut parsed_json: Value = if json_data.trim().is_empty() {
        // If the file is empty, initialize it with an empty JSON array
        json!([])
    } else {
        // Parse existing JSON data into a serde_json::Value
        serde_json::from_str(&json_data)?
    };

    let new_entry = json!({
        "id": clipboard_item.id,
        "item": clipboard_item.content,
    });

    parsed_json.as_array_mut().unwrap().push(new_entry);

    let updated_json = serde_json::to_string(&parsed_json)?;

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;
    file.write_all(updated_json.as_bytes())?;

    Ok(())
}