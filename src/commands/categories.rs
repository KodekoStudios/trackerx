use rusqlite::Connection;

pub fn run(conn: &Connection) {
    let mut stmt = conn
        .prepare("SELECT DISTINCT category FROM entries ORDER BY category ASC")
        .unwrap();
    let rows = stmt.query_map([], |r| r.get::<_, String>(0)).unwrap();

    let mut found = false;
    for r in rows {
        found = true;
        println!("{}", r.unwrap());
    }

    if !found {
        println!("no categories found");
    }
}
