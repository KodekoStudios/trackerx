use chrono::Utc;
use rusqlite::{Connection, ToSql};

pub fn run(conn: &Connection, category: Option<String>, days: Option<u32>) {
    let mut query = "SELECT id, timestamp, category, value, note FROM entries".to_string();
    let mut conditions = vec![];

    let mut _cat_str: Option<String> = None;
    let mut _from_ts: Option<i64> = None;

    let mut params_vec: Vec<&dyn ToSql> = vec![];

    if let Some(cat) = category {
        _cat_str = Some(cat);
        conditions.push("category = ?".to_string());
        params_vec.push(_cat_str.as_ref().unwrap());
    }

    if let Some(d) = days {
        let ts = Utc::now().timestamp() - (d as i64 * 86400);
        _from_ts = Some(ts);
        conditions.push("timestamp >= ?".to_string());
        params_vec.push(_from_ts.as_ref().unwrap());
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY timestamp DESC LIMIT 50");

    let mut stmt = conn.prepare(&query).unwrap();
    let rows = stmt
        .query_map(params_vec.as_slice(), |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, f64>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })
        .unwrap();

    let mut found = false;
    for r in rows {
        found = true;
        let (id, ts, cat, val, note) = r.unwrap();
        match note {
            Some(n) => println!("{id} {ts} {cat} {val} \"{n}\""),
            None => println!("{id} {ts} {cat} {val}"),
        }
    }

    if !found {
        println!("no entries found");
    }
}
