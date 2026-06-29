#[derive(Debug, Clone)]
pub struct TestSession {
    pub id: i64,
    pub test_id: i64,

    pub started_at: String,
    pub ended_at: Option<String>,

    pub reason: Option<String>,
}
