use actix_web::HttpRequest;

use crate::{errors::{AppError, TAppError}, models::user::User, db::{user::fetch_from_token, DbRef}};

pub async fn with_auth(req: &HttpRequest, db: &DbRef) -> Result<User, AppError> {
  // Bearer {token}
  let hdr = match req.headers().get("Authorization") {
    Some(v) => v.to_str().map_err(|err| AppError::auth_error(401, "INVALID_AUTH_HEADER", Some("auth header must be of type utf-8")))?,
    None => return Err(AppError::auth_error(401, "PROTECTED_ROUTE", Some("need credentials in Authorization header")))
  };

  let access_token = match hdr.trim().split(' ').last() {
    Some(v) => v,
    None => return Err(AppError::auth_error(401, "NO_ACCESS_TOKEN", Some("access token must exist in Authorization header, please note that the parser reads the last substring as the token")))
  };

  let user = fetch_from_token(access_token.to_string(), db).await?;

  Ok(user)
}