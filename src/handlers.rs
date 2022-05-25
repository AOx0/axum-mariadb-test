use crate::models::*;
use crate::state::State;
use axum::extract::Form;
use axum::response::Html;
use ddb::*;

pub async fn main_page() -> Html<&'static str> {
    Html(include_str!("../public/index.html"))
}

pub async fn login_page() -> Html<&'static str> {
    Html(include_str!("../public/login.html"))
}

pub async fn register_page() -> Html<&'static str> {
    Html(include_str!("../public/register.html"))
}

pub async fn register_user(Form(u): Form<UsersI>, state: Arc<State>) -> Html<&'static str> {
    use schema::Users::dsl as uu;

    let users: Vec<String> = uu::Users
        .filter(uu::email.eq(&u.email))
        .select(uu::email)
        .load::<String>(&mut state.pool.connect())
        .unwrap();

    if !users.is_empty() {
        Html("Error: This email is already registered!")
    } else if diesel::insert_into(Users::dsl::Users)
        .values(UsersI {
            name: u.name.clone(),
            email: u.email.clone(),
            password: u.password.clone(),
        })
        .execute(&mut state.pool.connect())
        .is_ok()
    {
        Html("User registered successfully!")
    } else {
        Html("Something falied while registering user!!")
    }
}

pub async fn login_user(
    Form(UsersLogin { email, password }): Form<UsersLogin>,
    state: Arc<State>,
) -> Html<&'static str> {
    use schema::Users::dsl as u;

    let users: Vec<String> = u::Users
        .filter(u::email.eq(&email))
        .filter(u::password.eq(&password))
        .select(u::email)
        .load::<String>(&mut state.pool.connect())
        .unwrap();

    if password.replace(" ", "").is_empty() || email.replace(" ", "").is_empty() {
        Html("Error: There are empty fields")
    } else if password.contains(" ") || email.contains(" ") {
        Html("Error: Fields can not have spaces")
    } else if users.len() == 1 {
        Html("Login successfull!")
    } else {
        Html("Invalid username/password")
    }
}
