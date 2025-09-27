use axum::{routing::get, Router};
use crate::handlers::{root, get_averages, get_classes, get_info, get_name, get_assignments, get_gradebook, get_weightings, get_report_card, get_progress_report};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/api/", get(root))
        .route("/api/name", get(get_name))
        .route("/api/info", get(get_info))
        .route("/api/classes", get(get_classes))
        .route("/api/averages", get(get_averages))
        .route("/api/assignments", get(get_assignments))
        .route("/api/gradebook", get(get_gradebook))
        .route("/api/weightings", get(get_weightings))
        .route("/api/reportcard", get(get_report_card))
        .route("/api/ipr", get(get_progress_report))
}

