#[derive(Debug, Clone)]
pub struct Sample {
    pub session_id: i64,
    pub sample_index: i64,

    pub timestamp: String,
    pub elapsed_ms: i64,

    pub voltage_mv: i64,
    pub current_ma: i64,
    pub capacity_mah: i64,
    pub energy_mwh: Option<i64>,
}
