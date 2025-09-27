use axum::{routing::get, Router};
use crate::handlers::{get_averages, get_classes, get_info, get_name, get_assignments, root};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/api/", get(root))
        .route("/api/name", get(get_name))
        .route("/api/info", get(get_info))
        .route("/api/classes", get(get_classes))
        .route("/api/averages", get(get_averages))
        .route("/api/assignments", get(get_assignments))
}

