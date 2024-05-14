extern crate x11_clipboard;
extern crate serde;
extern crate serde_json;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use x11_clipboard::Clipboard;
use serde::{Serialize, Deserialize};

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

            append_to_json(&clipboard_item);
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

fn append_to_json(clipboard_item: &ClipboardItem) {
    let path = "clipboard.json";

    // Open file with append mode or create it if it doesn't exist
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open clipboard.json");

    // Serialize clipboard_item to JSON
    let json = serde_json::to_string(clipboard_item).expect("Failed to serialize ClipboardItem");

    // Check if file is empty (i.e., if it's a new file)
    let is_empty = file.metadata().map(|m| m.len() == 0).unwrap_or(false);

    // If file is empty, write '[' to indicate the start of the JSON array
    if is_empty {
        writeln!(file, "[")
            .expect("Failed to write to clipboard.json");
    } else {
        // If file is not empty, prepend ',' to separate the new item
        writeln!(file, ",")
            .expect("Failed to write to clipboard.json");
    }

    // Write the serialized clipboard_item
    writeln!(file, "{}", json)
        .expect("Failed to write to clipboard.json");

    // Flush changes to file
    file.flush().expect("Failed to flush clipboard.json");
}
