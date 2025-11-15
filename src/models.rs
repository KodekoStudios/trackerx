use serde::Serialize;

#[derive(Serialize)]
pub struct Entry {
    pub id: i64,
    pub timestamp: i64,
    pub category: String,
    pub value: f64,
    pub note: Option<String>,
}
