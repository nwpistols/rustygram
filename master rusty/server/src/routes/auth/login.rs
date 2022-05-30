use crate::{
    io::error::Error,
    repos::UsersRepository,
    services::{Claims, Password, Token},
};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: String,
}

pub async fn login(
    body: web::Json<LoginRequest>,
    conn: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user_repository = UsersRepository { connection: &conn };

    let user = user_repository.find_user_with_email(&body.email).await?;

    if user.is_none() {
        return Ok(HttpResponse::BadRequest().body("Invalid email or password"));
    }

    let user = user.unwrap();

    let is_password_match =
        Password::verify_password(body.password.as_str(), user.password.as_str())?;

    if !is_password_match {
        return Ok(HttpResponse::BadRequest().body("Invalid email or password"));
    }

    let _date = Utc::now() + Duration::days(7);

    let token = Token::sign(Claims {
        sub: user.id,
        exp: _date.timestamp() as usize,
    })?;

    // Save jwt in cookie session
    session.insert("jwt", token)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        id: user.id,
        email: user.email,
        username: user.username,
    }))
}
