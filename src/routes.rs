use axum::{routing::get, Router};
use crate::handlers::{root, get_name, get_info, get_classes};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/api/name", get(get_name))
        .route("/api/info", get(get_info))
        .route("/api/classes", get(get_classes))
}

