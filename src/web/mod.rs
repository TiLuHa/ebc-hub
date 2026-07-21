pub mod handlers;
pub mod templates;
pub mod view_models;
pub mod validation;
pub mod forms;
pub mod error;
pub use error::AppError;

use std::sync::Arc;

use axum::{Router, routing::{get, post}};
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
            get(handlers::battery_type_detail).post(handlers::update_battery_type),
        )
        .route(
            "/batteries",
            get(handlers::batteries)
                .post(handlers::create_battery),
        )        .route(
            "/batteries/new",
            get(handlers::new_battery),
        )
        .route(
            "/batteries/{battery_id}",
            get(handlers::battery_detail)
                .post(handlers::update_battery),
        )
        .route(
            "/batteries/{battery_id}/intake",
            post(handlers::save_battery_intake),
        )
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
