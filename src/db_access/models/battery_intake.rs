#[derive(Debug)]
pub struct BatteryIntake {
    pub battery_id: String,
    pub serial_number: Option<String>,
    pub purchase_date: Option<String>,
    pub delivery_date: Option<String>,
    pub voltage_at_delivery_mv: Option<i64>,
    pub internal_resistance_at_delivery_uohm: Option<i64>,
    pub visual_inspection: Option<String>,
    pub notes: Option<String>,
}
