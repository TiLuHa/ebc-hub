#[derive(Debug, Clone)]
pub struct Battery {
    pub battery_id: String,
    pub battery_type_id: i64,

    pub notes: Option<String>,
}
