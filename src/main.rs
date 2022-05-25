use axum::{response::Html, routing::get, Router};

async fn main_page() -> Html<&'static str> {
    Html(include_str!("../public/index.html"))
}

async fn login_page() -> Html<&'static str> {
    Html(include_str!("../public/login.html"))
}

async fn register_page() -> Html<&'static str> {
    Html(include_str!("../public/register.html"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(main_page))
        .route("/login", get(login_page))
        .route("/register", get(register_page));

    axum::Server::bind(&std::net::SocketAddr::from(([127, 0, 0, 1], 8888)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
