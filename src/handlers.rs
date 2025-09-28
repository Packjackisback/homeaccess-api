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


pub async fn get_name(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let base_url = params.link.unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &base_url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
    };

    let html = match fetch_name_page(&client, &base_url).await {
        Ok(body) => body,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
    };

    match extract_name(&html) {
        Some(name) => (StatusCode::OK, Json(json!({ "name": name }))),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to parse name" })),
        ),
    }
}

pub async fn get_info(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params.link.unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (StatusCode::UNAUTHORIZED, Json(json!({"error": err})));
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))),
    };

    let html = match fetch_info_page(&client, &url).await {
        Ok(body) => body,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
    };

    match extract_info(&html) {
        Some(info) => (StatusCode::OK, Json(json!(info))),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to parse student info"})),
        ),
    }
}

pub async fn get_classes(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
    };

    let html = {
        let six_weeks_opt = params.six_weeks.clone();
        if let Some(six_weeks) = six_weeks_opt {
            match fetch_assignments_page_for_six_weeks(&client, &url, &six_weeks).await {
                Ok(body) => body,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            }
        } else {
            match fetch_assignments_page(&client, &url).await {
                Ok(body) => body,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            }
        }
    };

    let classes = extract_classes(&html, params.short.unwrap_or(false));

    (StatusCode::OK, Json(json!(classes)))
}


pub async fn get_averages(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

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
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                );
            }
        }
    } else {
        match fetch_assignments_page(&client, &url).await {
            Ok(body) => body,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                );
            }
        }
    };


    let averages = extract_averages(&html, params.short.unwrap_or(false));

    (StatusCode::OK, Json(json!(averages)))
}

pub async fn get_assignments(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": err })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            );
        }
    };

    let html = if let Some(six_weeks) = params.six_weeks.clone() {
        match fetch_assignments_page_for_six_weeks(&client, &url, &six_weeks).await {
            Ok(body) => body,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                );
            }
        }
    } else {
        match fetch_assignments_page(&client, &url).await {
            Ok(body) => body,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                );
            }
        }
    };

    let assignments = extract_assignments(&html, params.short.unwrap_or(false));

    (StatusCode::OK, Json(json!(assignments)))
}

pub async fn get_weightings(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": err })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            );
        }
    };

    let html = if let Some(six_weeks) = params.six_weeks.clone() {
        match fetch_assignments_page_for_six_weeks(&client, &url, &six_weeks).await {
            Ok(body) => body,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                );
            }
        }
    } else {
        match fetch_assignments_page(&client, &url).await {
            Ok(body) => body,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                );
            }
        }
    };
    let weightings = extract_weightings(&html, params.short.unwrap_or(false));

    (StatusCode::OK, Json(json!(weightings)))
}


pub async fn get_gradebook(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": err })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            );
        }
    };

    let html = if let Some(six_weeks) = &params.six_weeks {
        match fetch_assignments_page_for_six_weeks(&client, &url, six_weeks).await {
            Ok(body) => body,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                );
            }
        }
    } else {
        match fetch_assignments_page(&client, &url).await {
            Ok(body) => body,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                );
            }
        }
    };
    
    let gradebook = extract_gradebook(&html, params.short.unwrap_or(false));
    
    (StatusCode::OK, Json(json!(gradebook)))
}

pub async fn get_report_card(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": err })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            );
        }
    };

    let html = match fetch_report_page(&client, &url).await {
        Ok(body) => body,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
    };   

    let reportcard = extract_report_cards(&html);
    
    (StatusCode::OK, Json(json!(reportcard)))
}

pub async fn get_progress_report(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": err })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            );
        }
    };

    let html = match fetch_progress_page(&client, &url).await {
        Ok(body) => body,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
    };   

    let progressreport = extract_progress(&html);
    
    (StatusCode::OK, Json(json!(progressreport)))
}

pub async fn get_transcript(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": err })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            );
        }
    };

    let html = match fetch_transcript_page(&client, &url).await {
        Ok(body) => body,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
    };   

    let progressreport = extract_transcript(&html);
    
    (StatusCode::OK, Json(json!(progressreport)))
}

pub async fn get_rank(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params
        .link
        .clone()
        .unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": err })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            );
        }
    };

    let html = match fetch_transcript_page(&client, &url).await {
        Ok(body) => body,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
    };   

    let rank = extract_rank(&html);
    
    (StatusCode::OK, Json(json!(rank)))
}

