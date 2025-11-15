use chrono::Utc;
use dirs_next::data_local_dir;
use rusqlite::{Connection, params};
use std::fs;

pub fn open_db() -> Connection {
    let base = data_local_dir().expect("no local dir").join("trackerx");

    fs::create_dir_all(&base).unwrap();

    let path = base.join("trackerx.db");

    let conn = Connection::open(path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            category TEXT NOT NULL,
            value REAL NOT NULL,
            note TEXT
        )",
        [],
    )
    .unwrap();

    conn
}

pub fn insert_entry(conn: &Connection, category: &str, value: f64, note: Option<&str>) -> i64 {
    let ts = Utc::now().timestamp();

    conn.execute(
        "INSERT INTO entries (timestamp, category, value, note)
         VALUES (?1, LOWER(?2), ?3, ?4)",
        params![ts, category, value, note],
    )
    .unwrap();

    conn.last_insert_rowid()
}

pub fn entry_exists(conn: &Connection, category: &str, value: f64, note: Option<&str>) -> bool {
    let exists: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM entries
             WHERE LOWER(category) = LOWER(?1) AND value = ?2 AND note IS ?3",
            params![category, value, note],
            |row| row.get(0),
        )
        .unwrap_or(0);

    exists > 0
}
