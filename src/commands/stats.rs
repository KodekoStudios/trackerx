use rusqlite::Connection;

pub fn run(conn: &Connection, category: String) {
    let sql = "
        SELECT
            SUM(value),
            AVG(value),
            MAX(value),
            MIN(value)
        FROM entries
        WHERE category = ?
    ";

    let mut stmt = conn.prepare(sql).unwrap();

    let row = stmt
        .query_row([category.as_str()], |r| {
            Ok((
                r.get::<_, Option<f64>>(0)?,
                r.get::<_, Option<f64>>(1)?,
                r.get::<_, Option<f64>>(2)?,
                r.get::<_, Option<f64>>(3)?,
            ))
        })
        .unwrap();

    let (sum, avg, max, min) = row;

    if sum.is_none() {
        println!("no entries for '{}'", category);
        return;
    }

    println!("stats for '{}'", category);
    println!("total: {}", sum.unwrap());
    println!("daily avg: {}", avg.unwrap());
    println!("max: {}", max.unwrap());
    println!("min: {}", min.unwrap());
}
