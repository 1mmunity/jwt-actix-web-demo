use std::sync::{RwLock, Arc, RwLockWriteGuard, RwLockReadGuard};

use redis::{Connection, Commands};

use crate::{errors::{AppError, TAppError}, auth::tokens::JWTClaim};

pub type RefreshTokenStore = Arc<RwLock<RefreshTokenRedis>>;

pub struct RefreshTokenRedis {
  pub logger: slog::Logger,
  pub client: redis::Client
}

impl RefreshTokenRedis {
  pub fn init_client(logger: slog::Logger, host: String) -> Result<Self, AppError> {
    info!(logger, "> Initialized redis client");
    let client = redis::Client::open(host)
    .map_err(|_| AppError::crit_error(logger.clone(), "CANNOT_INIT_REDIS", Some("unable to initialize redis client")))?;

    let slf = RefreshTokenRedis{
      logger,
      client
    };
    
    Ok(slf)
  }

  pub fn get_conn(&self) -> Result<Connection, AppError> {
    self.client.get_connection()
    .map_err(|e| AppError::crit_error(self.logger.clone(), "CANNOT_CONN_REDIS", Some("unable to connect to redis server")))
  }

  pub fn init_from_env(logger: slog::Logger) -> Result<Self, AppError> {
    RefreshTokenRedis::init_client(logger, dotenv!("REDIS_URL").into())
  }

  pub fn get_writable(logger: slog::Logger, rts: &RefreshTokenStore) -> Result<RwLockWriteGuard<'_, Self>, AppError> {
    rts.write()
    .map_err(|e| AppError::crit_error(logger, "RTS_VAR_POIS", Some("rts variable poisoned")))
  }

  pub fn get_readable(logger: slog::Logger, rts: &RefreshTokenStore) -> Result<RwLockReadGuard<'_, Self>, AppError> {
    rts.read()
    .map_err(|e| AppError::crit_error(logger, "RTS_VAR_POIS", Some("rts variable poisoned")))
  }

  /**
   * refreshes all tokens, returns new access token and new refresh token
   */
  pub fn refresh_tokens(logger: slog::Logger, rts: &RefreshTokenStore, old_reftok: String) -> Result<(String, String), AppError> {
    let user_id = JWTClaim::verify_refresh_token(old_reftok.clone())?.to_string();
    let dbread = RefreshTokenRedis::get_readable(logger.clone(), rts)?;

    let mut conn = dbread.get_conn()?;
    let curr_reftok: String = match conn.get(user_id.clone())
    .map_err(|err| AppError::crit_error(logger.clone(), "CANNOT_GET_REDIS", Some(err.to_string())))? {
      Some(v) => v,
      None => return Err(AppError::auth_error(401, "NO_REFRESH_TOKEN", Some("you have an invalid refresh token")))
    };
  

    if curr_reftok != old_reftok {
      return Err(AppError::auth_error(401, "INVALID_REFRESH_TOKEN", Some("your refresh token has been cycled")));
    }

    let new_reftok = JWTClaim::refresh_token(logger.clone(), rts, user_id.clone())?;
    let new_acctok = JWTClaim::access_token(user_id)?;

    Ok((new_acctok, new_reftok))
  }
}