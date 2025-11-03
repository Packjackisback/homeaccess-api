use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    response::IntoResponse,
    http::header,
};
use serde_json::json;
use serde::Deserialize;
use crate::auth::login_handler;
use crate::cache::Cache;
use crate::scraping::{extract_assignments, extract_averages, extract_classes, extract_gradebook, extract_info, extract_name, extract_report_cards, extract_weightings, extract_progress, extract_transcript, extract_rank};
use crate::fetchers::{fetch_info_page, fetch_assignments_page, fetch_name_page, fetch_assignments_page_for_six_weeks, fetch_report_page, fetch_progress_page, fetch_transcript_page};

#[derive(Deserialize)]
pub struct LoginParams {
    pub user: String,
    pub pass: String,
    pub link: Option<String>,
    pub short: Option<bool>,
    pub six_weeks: Option<String>,
    pub no_cache: Option<bool>,
}

async fn get_or_login(
    cache: &Cache,
    username: &str,
    password: &str,
    url: &str,
    no_cache: bool,
) -> Result<reqwest::Client, String> {
    if !no_cache {
        if let Some(client) = cache.get_client(username, url).await {
            return Ok(client);
        }
    }

    let client = login_handler(username, password, url).await?;
    
    if !no_cache {
        cache.set_client(username, url, client.clone()).await;
    }
    
    Ok(client)
}

macro_rules! endpoint {
    (
        $name:ident,
        assignments_page_scraper: $extract_fn:path
    ) => {
        pub async fn $name(
            State(cache): State<Cache>,
            Query(params): Query<LoginParams>
        ) -> impl IntoResponse {
            let url = params.link.clone().unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());
            let no_cache = params.no_cache.unwrap_or(false);

            let client = match get_or_login(&cache, &params.user, &params.pass, &url, no_cache).await {
                Ok(c) => c,
                Err(err) if err == "Invalid username or password" => {
                    return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let html = match if let Some(ref six_weeks) = params.six_weeks {
                fetch_assignments_page_for_six_weeks(&client, &url, six_weeks).await
            } else {
                fetch_assignments_page(&client, &url, &cache, &params.user, no_cache).await
            } {
                Ok(body) => body,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
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
        pub async fn $name(
            State(cache): State<Cache>,
            Query(params): Query<LoginParams>
        ) -> impl IntoResponse {
            let url = params.link.clone().unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());
            let no_cache = params.no_cache.unwrap_or(false);

            let client = match get_or_login(&cache, &params.user, &params.pass, &url, no_cache).await {
                Ok(c) => c,
                Err(err) if err == "Invalid username or password" => {
                    return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let html = match $fetch_fn(&client, &url, &cache, &params.user, no_cache).await {
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
        pub async fn $name(
            State(cache): State<Cache>,
            Query(params): Query<LoginParams>
        ) -> impl IntoResponse {
            let url = params.link.clone().unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());
            let no_cache = params.no_cache.unwrap_or(false);

            let client = match get_or_login(&cache, &params.user, &params.pass, &url, no_cache).await {
                Ok(c) => c,
                Err(err) if err == "Invalid username or password" => {
                    return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let html = match $fetch_fn(&client, &url, &cache, &params.user, no_cache).await {
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
        pub async fn $name(
            State(cache): State<Cache>,
            Query(params): Query<LoginParams>
        ) -> impl IntoResponse {
            let url = params.link.clone().unwrap_or_else(|| "https://homeaccess.katyisd.org".to_string());
            let no_cache = params.no_cache.unwrap_or(false);

            let client = match get_or_login(&cache, &params.user, &params.pass, &url, no_cache).await {
                Ok(c) => c,
                Err(err) if err == "Invalid username or password" => {
                    return (StatusCode::UNAUTHORIZED, Json(json!({ "error": err })));
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))),
            };

            let html = match $fetch_fn(&client, &url, &cache, &params.user, no_cache).await {
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
        "message": "Interactive API documentation available at /docs",
        "docs_url": "https://hac.packjack.dev/docs",
        "openapi_spec": {
            "yaml": "https://hac.packjack.dev/openapi.yaml",
            "json": "https://hac.packjack.dev/openapi.json"
        },
        "routes": [
            "/api/name", "/api/assignments", "/api/info", "/api/averages", "/api/weightings", "/api/classes", "/api/reportcard", "/api/ipr", "/api/transcript", "/api/rank"
        ],
        "cache_param": "Add ?no_cache=true to any endpoint to bypass cache"
    });
    Json(message)
}

pub async fn serve_openapi_yaml() -> impl IntoResponse {
    let openapi_content = include_str!("../openapi.yaml");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/yaml")],
        openapi_content
    )
}

pub async fn serve_openapi_json() -> impl IntoResponse {
    let openapi_content = include_str!("../openapi.json");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        openapi_content
    )
}

pub async fn serve_docs() -> impl IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Home Access Center API Documentation</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui.css">
    <style>
        body {
            margin: 0;
            padding: 0;
        }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {
            window.ui = SwaggerUIBundle({
                url: "/openapi.yaml",
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            });
        };
    </script>
</body>
</html>
"#;
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html
    )
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
