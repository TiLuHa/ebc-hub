#[derive(Debug, Clone)]
pub struct TestSession {
    pub id: String,
    pub test_id: String,

    pub started_at: String,
    pub ended_at: Option<String>,

    pub reason: Option<String>,
}
