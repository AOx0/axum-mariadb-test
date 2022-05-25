mod handlers;

use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(handlers::main_page))
        .route("/login", get(handlers::login_page))
        .route("/register", get(handlers::register_page));

    axum::Server::bind(&std::net::SocketAddr::from(([127, 0, 0, 1], 8888)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
