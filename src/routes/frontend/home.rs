use axum::response::{Html, IntoResponse, Redirect, Response};

use crate::{auth::User, error::AppError};

pub async fn page(maybe_user: Option<User>) -> Result<Response, AppError> {
    let Some(user) = maybe_user else {
        return Ok(Redirect::to("/login").into_response());
    };

    Ok(Html(format!("Hello {}!", user.username())).into_response())
}
