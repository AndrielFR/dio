use askama::Template;
use axum::{
    Form, Router,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
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

async fn login(
    repo: Repository,
    jar: CookieJar,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let guest_user = GuestUser::new(request.username, request.password);
    let user = match guest_user.authenticate(&repo).await {
        Ok(u) => u,
        Err(AppError::UserDoesNotExists) => guest_user.register(&repo).await?,
        Err(e) => return Err(e),
    };

    let token = user.auth_token()?;
    let cookie = Cookie::build(("token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/")))
}
