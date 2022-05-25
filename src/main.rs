mod handlers;
mod models;
mod state;

use axum::routing::get;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(state::State {
        pool: ddb::create_pool(),
    });

    let app = axum::Router::new()
        .route("/", get(handlers::main_page))
        .route(
            "/login",
            get(handlers::login_page).post({
                let shared_state = Arc::clone(&shared_state);
                move |user| handlers::login_user(user, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/register",
            get(handlers::register_page).post({
                let shared_state = Arc::clone(&shared_state);
                move |user| handlers::register_user(user, Arc::clone(&shared_state))
            }),
        );

    axum::Server::bind(&std::net::SocketAddr::from(([127, 0, 0, 1], 8888)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
