mod routes;
mod handlers;
mod auth;
mod scraping;
mod fetchers;
mod cache;

use std::net::SocketAddr;
use cache::Cache;
use std::time::Duration;
use lambda_http::{service_fn, Body as LambdaBody, Request as LambdaRequest, Request, Response as LambdaResponse, run};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let login_cache = 30 * 60;
    let page_cache = 5 * 60;
    let cache = Cache::new(login_cache, page_cache);
    
    let cache_cleaner = cache.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10 * 60));
        loop {
            interval.tick().await;
            cache_cleaner.clear_expired().await;
        }
    });

    let app = routes::create_router(cache);

    if std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {
        println!("Running in AWS Lambda environment");
        axum_lambda::run(app).await?;
       Ok(())
    } else {
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        println!("Listening on http://{}", addr);
        println!("Cache configuration:");
        println!("  - Login sessions: {} minutes", login_cache);
        println!("  - Page data: {} minutes", page_cache);

        axum::serve(
            tokio::net::TcpListener::bind(addr).await.unwrap(),
            app.into_make_service(),
        )
        .await
        .unwrap();
        Ok(())
    }
}
