use std::sync::Arc;
use axum::{Router, routing::{get}};

use crate::{AppState, handlers::health_checker::{health_checker}};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/auth/test", get(health_checker))
        .with_state(app_state)
}

