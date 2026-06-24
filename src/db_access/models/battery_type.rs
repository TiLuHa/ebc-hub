#[derive(Debug, Clone)]
pub struct BatteryType {
    pub id: i64,
    pub manufacturer: String,
    pub model: String,
    pub chemistry: String,
    pub nominal_voltage_mv: i64,
    pub nominal_capacity_mah: i64,
    pub charge_termination_voltage_mv: i64,
    pub discharge_cutoff_voltage_mv: i64,
    pub notes: Option<String>,
}
