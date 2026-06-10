use askama::Template;
use axum::{Form, Router, response::Html, routing::get};
use serde::Deserialize;

use crate::{app::AppState, auth::user::GuestUser, error::AppError, repository::Repository};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(page).post(login))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(repo: Repository, Form(request): Form<LoginForm>) -> Result<Html<String>, AppError> {
    let guest_user = GuestUser::new(request.username, request.password);
    let user = match guest_user.authenticate(&repo).await {
        Ok(u) => u,
        Err(AppError::UserDoesNotExists) => guest_user.register(&repo).await?,
        Err(e) => return Err(e),
    };

    Ok(Html(user.username().clone()))
}
