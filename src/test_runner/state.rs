use crate::db_access::models::Test;

#[derive(Debug)]
pub enum State {
    Idle,
    Running { test: Test, session_id: i64 },
    Failed,
    Paused,
    Stopped { test: Test, session_id: i64 },
    Finished,
}
