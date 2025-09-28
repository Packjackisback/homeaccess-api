use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    response::IntoResponse,
};
use serde_json::json;
use serde::Deserialize;
use crate::auth::login_handler;
use crate::scraping::{extract_assignments, extract_averages, extract_classes, extract_gradebook, extract_info, extract_name, extract_report_cards, extract_weightings, extract_progress, extract_transcript, extract_rank};
use crate::fetchers::{fetch_info_page, fetch_assignments_page, fetch_name_page, fetch_assignments_page_for_six_weeks, fetch_report_page, fetch_progress_page, fetch_transcript_page};

#[derive(Deserialize)]
pub struct LoginParams {
    pub user: String,
    pub pass: String,
    pub link: Option<String>,
    pub short: Option<bool>,
    pub six_weeks: Option<String>,
}

macro_rules! endpoint {
    (
        $name:ident,
        assignments_page_scraper: $extract_fn:path
    ) => {
        pub async fn $name(Query(params): Query<LoginParams>) -> impl IntoResponse {
            let url = params.link.clone().unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

            let client = match login_handler(&params.user, &params.pass, &url).await {
                Ok(c) => c,
                Err(err) if err == "Invalid username or password" => {
                    return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let html = if let Some(six_weeks) = params.six_weeks.clone() {
                match fetch_assignments_page_for_six_weeks(&client, &url, &six_weeks).await {
                    Ok(body) => body,
                    Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
                }
            } else {
                match fetch_assignments_page(&client, &url).await {
                    Ok(body) => body,
                    Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
                }
            };

            let data = $extract_fn(&html, params.short.unwrap_or(false));

            (StatusCode::OK, Json(json!(data)))
        }
    };


    (
        $name:ident,
        single_page: $fetch_fn:path,
        $extract_fn:path,
        error_msg: $error_msg:expr,
        key: name
    ) => {
        pub async fn $name(Query(params): Query<LoginParams>) -> impl IntoResponse {
            let url = params.link.clone().unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

            let client = match login_handler(&params.user, &params.pass, &url).await {
                Ok(c) => c,
                Err(err) if err == "Invalid username or password" => {
                    return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let html = match $fetch_fn(&client, &url).await {
                Ok(body) => body,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            match $extract_fn(&html) {
                Some(data) => (StatusCode::OK, Json(json!({ "name": data }))),
                None => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": $error_msg })),
                ),
            }
        }
    };
    (
        $name:ident,
        single_page: $fetch_fn:path,
        $extract_fn:path,
        error_msg: $error_msg:expr
    ) => {
        pub async fn $name(Query(params): Query<LoginParams>) -> impl IntoResponse {
            let url = params.link.clone().unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

            let client = match login_handler(&params.user, &params.pass, &url).await {
                Ok(c) => c,
                Err(err) if err == "Invalid username or password" => {
                    return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let html = match $fetch_fn(&client, &url).await {
                Ok(body) => body,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            match $extract_fn(&html) {
                Some(data) => (StatusCode::OK, Json(json!(data))),
                None => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": $error_msg })),
                ),
            }
        }
    };

    (
        $name:ident,
        vec_result: $fetch_fn:path,
        $extract_fn:path
    ) => {
        pub async fn $name(Query(params): Query<LoginParams>) -> impl IntoResponse {
            let url = params.link.clone().unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

            let client = match login_handler(&params.user, &params.pass, &url).await {
                Ok(c) => c,
                Err(err) if err == "Invalid username or password" => {
                    return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let html = match $fetch_fn(&client, &url).await {
                Ok(body) => body,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let data = $extract_fn(&html);

            (StatusCode::OK, Json(json!(data)))
        }
    };
}



pub async fn root() -> impl IntoResponse {
    let message = json!({
        "title": "Welcome to the Home Access Center API!",
        "message": "Visit the docs at https://homeaccesscenterapi-docs.vercel.app/",
        "routes": [
            "/api/name", "/api/assignments", "/api/info", "/api/averages",
            "/api/classes", "/api/reportcard", "/api/ipr", "/api/transcript", "/api/rank"
        ]
    });
    Json(message)
}

endpoint!(
    get_classes,
    assignments_page_scraper: extract_classes
);

endpoint!(
    get_averages,
    assignments_page_scraper: extract_averages
);

endpoint!(
    get_assignments,
    assignments_page_scraper: extract_assignments
);

endpoint!(
    get_weightings,
    assignments_page_scraper: extract_weightings
);

endpoint!(
    get_gradebook,
    assignments_page_scraper: extract_gradebook
);

endpoint!(
    get_name,
    single_page: fetch_name_page,
    extract_name,
    error_msg: "Failed to parse name",
    key: name
);

endpoint!(
    get_info,
    single_page: fetch_info_page,
    extract_info,
    error_msg: "Failed to parse student info"
);

endpoint!(
    get_report_card,
    vec_result: fetch_report_page,
    extract_report_cards
);

endpoint!(
    get_progress_report,
    vec_result: fetch_progress_page,
    extract_progress
);

endpoint!(
    get_transcript,
    vec_result: fetch_transcript_page,
    extract_transcript
);

endpoint!(
    get_rank,
    vec_result: fetch_transcript_page,
    extract_rank
);

