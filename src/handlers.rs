use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    response::IntoResponse,
};
use serde_json::json;
use serde::Deserialize;

use crate::auth::login_handler;
use crate::scraping::extract_name;
use crate::scraping::extract_info;
use crate::scraping::extract_classes;

#[derive(Deserialize)]
pub struct LoginParams {
    pub user: String,
    pub pass: String,
    pub link: Option<String>,
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
    let url = params.link.unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());
    
    let result: Result<String, (StatusCode, Json<serde_json::Value>)> = async {
        let client = login_handler(&params.user, &params.pass, &url)
            .await
            .map_err(|err| {
                if err == "Invalid username or password" {
                    (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })))
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err })))
                }
            })?;

        let endpoint = format!("{}/HomeAccess/Classes/Classwork", url);
        let content = client.get(&endpoint)
            .send().await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to fetch classwork page" }))))?
            .text().await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to read response body" }))))?;
        
        extract_name(&content).ok_or_else(|| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to parse name" })))
        })
    }.await;

    match result {
        Ok(name) => (StatusCode::OK, Json(json!({ "name": name }))),
        Err(err_response) => err_response,
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

    let endpoint = format!("{}/HomeAccess/Content/Student/Registration.aspx", url);
    let response = match client.get(&endpoint).send().await {
        Ok(resp) => resp,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch registration page"})),
            )
        }
    };
    let body = match response.text().await {
        Ok(text) => text,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to read response body"})),
            )
        }
    };

    match extract_info(&body) {
        Some(info) => (StatusCode::OK, Json(json!(info))),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to parse student info"})),
        ),
    }
}

pub async fn get_classes(Query(params): Query<LoginParams>) -> impl IntoResponse {
    let url = params.link.unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());

    let client = match login_handler(&params.user, &params.pass, &url).await {
        Ok(c) => c,
        Err(err) if err == "Invalid username or password" => {
            return (StatusCode::UNAUTHORIZED, Json(json!({"error": err})));
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e}))),
    };

    let endpoint = format!("{}/HomeAccess/Content/Student/Assignments.aspx", url);
    let response = match client.get(&endpoint).send().await {
        Ok(resp) => resp,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch assignments page"})),
            )
        }
    };

    let body = match response.text().await {
        Ok(text) => text,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to read response body"})),
            )
        }
    };

    let classes = extract_classes(&body);
    (StatusCode::OK, Json(json!(classes)))
}
