mod routes;
mod handlers;
mod auth;
mod scraping;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = routes::create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

