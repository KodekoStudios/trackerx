use crate::models::Entry;
use rusqlite::Connection;
use std::fs;

pub fn run(conn: &Connection, path: String) {
    let mut stmt = conn
        .prepare("SELECT id, timestamp, category, value, note FROM entries ORDER BY timestamp")
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok(Entry {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                category: row.get(2)?,
                value: row.get(3)?,
                note: row.get(4)?,
            })
        })
        .unwrap();

    let mut data = vec![];
    for r in rows {
        data.push(r.unwrap());
    }

    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::write(&path, json).unwrap();

    println!("exported on {path}");
}
