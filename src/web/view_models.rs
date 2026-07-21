use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct BatteryListItem {
    pub battery_id: String,
    pub battery_type_id: i64,

    pub manufacturer: String,
    pub model: String,
    pub chemistry: String,

    pub battery_notes: Option<String>,

    pub has_intake: bool,
}