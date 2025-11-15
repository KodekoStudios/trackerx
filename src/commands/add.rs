use crate::db::{entry_exists, insert_entry};
use rusqlite::Connection;

pub fn run(conn: &Connection, category: String, value: f64, note: Option<String>) {
    let category_norm = category.trim().to_lowercase();

    if category_norm.is_empty() {
        println!("category cannot be empty");
        return;
    }

    if value == 0.0 || value.is_nan() || value.is_infinite() {
        println!("value must be a valid non-zero number");
        return;
    }

    if let Some(ref n) = note {
        if n.trim().is_empty() {
            println!("note cannot be empty if provided");
            return;
        }
    }

    if entry_exists(conn, &category_norm, value, note.as_deref()) {
        println!("this entry already exists");
        return;
    }

    let id = insert_entry(conn, &category_norm, value, note.as_deref());
    println!(
        "added entry #{} to '{}' with value {}",
        id, category_norm, value
    );
}
