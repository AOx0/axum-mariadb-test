use axum::response::Html;

pub async fn main_page() -> Html<&'static str> {
    Html(include_str!("../public/index.html"))
}

pub async fn login_page() -> Html<&'static str> {
    Html(include_str!("../public/login.html"))
}

pub async fn register_page() -> Html<&'static str> {
    Html(include_str!("../public/register.html"))
}
