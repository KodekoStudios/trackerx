use chrono::Utc;
use rusqlite::{Connection, ToSql};

pub fn run(conn: &Connection, id: Option<i64>, category: Option<String>, days: Option<u32>) {
    let mut query = "DELETE FROM entries".to_string();
    let mut conditions = vec![];

    let mut _cat_str: Option<String> = None;
    let mut _from_ts: Option<i64> = None;
    let mut _id_val: Option<i64> = None;

    let mut params_vec: Vec<&dyn ToSql> = vec![];

    if let Some(i) = id {
        _id_val = Some(i);
        conditions.push("id = ?".to_string());
        params_vec.push(_id_val.as_ref().unwrap());
    }

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

    let mut stmt = conn.prepare(&query).unwrap();
    let affected = stmt.execute(params_vec.as_slice()).unwrap();

    if affected == 0 {
        println!("no matching entries to delete");
    } else {
        println!("deleted {affected} entries");
    }
}
