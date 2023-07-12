use crate::{
  models::user::User,
  errors::{AppError, TAppError},
  auth::tokens::JWTClaim,
};

use super::{Db, DbRef, redis::RefreshTokenStore};

#[derive(Deserialize)]
pub struct UserCredentials {
  pub email: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct UserCreate {
  pub username: String,
  pub email: String,
  pub password: String,
}

pub async fn fetch_from_token(token: String, db: &DbRef) -> Result<User, AppError> {
  let dbwrite = Db::get_writable(db)?;
  let user_id = JWTClaim::verify_access_token(token)?;
  let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
  .fetch_one(&dbwrite.pool)
  .await
  .map_err(|err| {
    match err {
      sqlx::Error::RowNotFound => AppError::db_error(401, "USER_NOT_FOUND", Some("user not found with the given credentials")),
      _ => AppError::crit_error(dbwrite.logger.clone(), "UNKNOWN_USER_ERROR", Some("user cannot be fetched, please try again later."))
    }
  })?;

  Ok(user)
}

pub async fn user_login(rts: &RefreshTokenStore, db: &DbRef, creds: &UserCredentials) -> Result<(User, (String, String)), AppError> {
  let dbwrite = Db::get_writable(db)?;
  
  let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1 AND password = crypt($2, password)", creds.email, creds.password)
  .fetch_one(&dbwrite.pool)
  .await
  .map_err(|err| {
    match err {
      sqlx::Error::RowNotFound => AppError::db_error(401, "USER_NOT_FOUND", Some("user not found with the given credentials")),
      _ => AppError::crit_error(dbwrite.logger.clone(), "UNKNOWN_USER_ERROR", Some("user cannot be fetched, please try again later."))
    }
  })?;

  let access_token = JWTClaim::access_token(user.id.to_string())?;
  let refresh_token = JWTClaim::refresh_token(dbwrite.logger.clone(), rts, user.id.to_string())?;
  Ok((user, (access_token, refresh_token)))
}

pub async fn user_create(rts: &RefreshTokenStore, db: &DbRef, new_user: &UserCreate) -> Result<(User, (String, String)), AppError> {
  let dbwrite = Db::get_writable(db)?;
  let user = sqlx::query_as!(User, "INSERT INTO users (username, email, password) VALUES ($1, $2, crypt($3, gen_salt('bf'))) RETURNING *", new_user.username, new_user.email, new_user.password)
  .fetch_one(&dbwrite.pool)
  .await
  .map_err(|err| {
    match err {
      sqlx::Error::Database(dberr) => AppError::db_error(400, "ERR_FROM_DB", Some(dberr.to_string())),
      _ => AppError::crit_error(dbwrite.logger.clone(), "UNKNOWN_USER_ERROR", Some(err.to_string()))
    }
  })?;

  let access_token = JWTClaim::access_token(user.id.to_string())?;
  let refresh_token = JWTClaim::refresh_token(dbwrite.logger.clone(), rts, user.id.to_string())?;
  Ok((user, (access_token, refresh_token)))
}