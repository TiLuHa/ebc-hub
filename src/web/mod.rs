pub mod handlers;
pub mod templates;

use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::db_access::Storage;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<Storage>,
    // pub ebc_manager: EbcManagerHandle,
    // pub test_runner: TestRunnerHandle,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .route(
            "/battery-types",
            get(handlers::battery_types).post(handlers::create_battery_type),
        )
        .route("/battery-types/new", get(handlers::new_battery_type))
        .route(
            "/battery-types/{battery_type_id}",
            get(handlers::battery_type_detail),
        )
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
