use actix_web::{
  Responder,
  web,
  cookie::{Cookie, time::Duration, SameSite},
  HttpRequest
};
use serde_json::json;

use crate::{db::{DbRef, self, user::{UserCredentials, UserCreate}, redis::{RefreshTokenStore, RefreshTokenRedis}}, errors::{AppError, TAppError}, models::user::User, middlewares::auth::with_auth, utils::logger::LoggerRef};

pub fn auth_scope() -> actix_web::Scope {
  web::scope("/auth")
  .service(login_user)
  .service(register_user)
  .service(me)
  .service(refresh_access_token)
}

#[derive(Serialize)]
struct UserSigninRes {
  user: User,
  access_token: String
}

#[get("/me")]
async fn me(req: HttpRequest, db: web::Data<DbRef>) -> Result<impl Responder, AppError> {
  let user = with_auth(&req, &db).await?;
  Ok(ok_res!(200, user))
}

#[post("/login")]
async fn login_user(rts: web::Data<RefreshTokenStore>, db: web::Data<DbRef>, body: web::Json<UserCredentials>) -> Result<impl Responder, AppError> {
  let user = db::user::user_login(&rts, &db, &body.0)
  .await?;

  let rtcookie = Cookie::build("jwt", user.1.1)
  .same_site(SameSite::None)
  .http_only(true)
  .secure(true)
  .max_age(Duration::days(2))
  .finish();

  Ok(
    ok_res!(200, UserSigninRes{
      user: user.0,
      access_token: user.1.0
    }, rtcookie)
  )
}

#[post("/register")]
async fn register_user(rts: web::Data<RefreshTokenStore>, db: web::Data<DbRef>, body: web::Json<UserCreate>) -> Result<impl Responder, AppError> {
  let user = db::user::user_create(&rts, &db, &body.0)
  .await?;

  let rtcookie = Cookie::build("jwt", user.1.1)
  .same_site(SameSite::None)
  .http_only(true)
  .secure(true)
  .max_age(Duration::days(2))
  .finish();

  Ok(
    ok_res!(200, UserSigninRes{
      user: user.0,
      access_token: user.1.0
    }, rtcookie)
  )
}

#[get("/refreshAccessToken")]
async fn refresh_access_token(req: HttpRequest, rts: web::Data<RefreshTokenStore>, logger: web::Data<LoggerRef>) -> Result<impl Responder, AppError> {
  let cookies = req.cookies()
  .map_err(|e| AppError::auth_error(400, "CANNOT_PARSE_COOKIES", Some(e.to_string())))?;
  let jwt_cookie = match cookies.iter().find(|cookie| cookie.name() == "jwt".to_string()) {
    Some(v) => v,
    None => return Err(AppError::auth_error(401, "NO_JWT_REFTOKEN_COOKIE", Some("no jwt cookie in header")))
  };

  let logger = logger.clone();
  let logger = logger.write().unwrap();
  
  let tokens = RefreshTokenRedis::refresh_tokens(logger.clone(), &rts, jwt_cookie.value().to_string())?;

  let new_jwt_cookie = Cookie::build("jwt", tokens.1).finish();
  Ok(ok_res!(
    200,
    json!({
      "access_token": tokens.0
    }),
    new_jwt_cookie
  ))
}