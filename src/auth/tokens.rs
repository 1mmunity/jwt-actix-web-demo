use std::str::FromStr;
use redis::Commands;

use jsonwebtoken::{Header, EncodingKey, DecodingKey, Validation};
use uuid::Uuid;

use crate::{errors::{AppError, TAppError}, db::redis::{RefreshTokenStore, RefreshTokenRedis}};

const ACCESS_TOKEN_SECRET: &[u8] =dotenv!("ACCESS_TOKEN_SECRET").as_bytes();
const REFRESH_TOKEN_SECRET: &[u8] =dotenv!("REFRESH_TOKEN_SECRET").as_bytes();

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaim {
  exp: u64,
  user_id: String,
}

impl JWTClaim {
  pub fn access_token(user_id: String) -> Result<String, AppError> {
    let claim = JWTClaim {
      exp: jsonwebtoken::get_current_timestamp() + 300,
      user_id,
    };

    jsonwebtoken::encode(&Header::default(), &claim, &EncodingKey::from_secret(ACCESS_TOKEN_SECRET))
    .map_err(|e| AppError::auth_error(500, "CANNOT_ENC_JWT_ACC", Some(e.to_string())))
  }

  pub fn refresh_token(logger: slog::Logger, rts: &RefreshTokenStore, user_id: String) -> Result<String, AppError> {
    let claim = JWTClaim {
      exp: jsonwebtoken::get_current_timestamp() + 172800,
      user_id: user_id.clone(),
    };

    let tok = jsonwebtoken::encode(&Header::default(), &claim, &EncodingKey::from_secret(REFRESH_TOKEN_SECRET))
    .map_err(|e| AppError::auth_error(500, "CANNOT_ENC_JWT_REF", Some(e.to_string())))?;

    let rts = RefreshTokenRedis::get_readable(logger, rts)?;

    let mut conn = rts.get_conn()?;

    conn.set(user_id, tok.clone())
    .map_err(|_| AppError::crit_error(rts.logger.clone(), "CANNOT_SET_REFTOK", Some("cannot set ref token in redis")))?;

    Ok(tok)
  }

  /**
   * returns user id
   */
  pub fn verify_access_token(at: String) -> Result<Uuid, AppError> {
    let t = jsonwebtoken::decode::<JWTClaim>(&at, &DecodingKey::from_secret(ACCESS_TOKEN_SECRET), &Validation::default())
    .map_err(|err| match err.into_kind() {
      jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::auth_error(401, "EXPIRED_ACCESS_TOKEN", Some("your access token has expired, please refresh at /auth/refreshAccessToken")),
      _ => AppError::auth_error(401, "INVALID_ACCESS_TOKEN", Some("cannot decode access token"))
    })?;

    let user_id = Uuid::from_str(&t.claims.user_id)
    .map_err(|err| AppError::auth_error(401, "INVALID_UID_ACCESS_TOKEN", Some("invalid user_id in access token")))?;
    Ok(user_id)
  }

  /**
   * returns user id
   */
  pub fn verify_refresh_token(rt: String) -> Result<Uuid, AppError> {
    let t = jsonwebtoken::decode::<JWTClaim>(&rt, &DecodingKey::from_secret(REFRESH_TOKEN_SECRET), &Validation::default())
    .map_err(|err| match err.into_kind() {
      jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::auth_error(401, "EXPIRED_REFRESH_TOKEN", Some("your refresh token has expired, please login again")),
      _ => AppError::auth_error(401, "INVALID_ACCESS_TOKEN", Some("cannot decode refresh token"))
    })?;

    let user_id = Uuid::from_str(&t.claims.user_id)
    .map_err(|err| AppError::auth_error(401, "INVALID_UID_REFRESH_TOKEN", Some("invalid user_id in refresh token")))?;
    Ok(user_id)
  }
}