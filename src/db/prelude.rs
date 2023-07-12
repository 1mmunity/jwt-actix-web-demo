use std::{time::Duration, sync::{Arc, RwLock, RwLockWriteGuard, RwLockReadGuard}};

use slog::Logger;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::errors::{AppError, TAppError};

pub type DbRef = Arc<RwLock<Db>>;

pub struct Db {
  pub logger: Logger,
  pub pool: Pool<Postgres>,
}

const DBCONN_TIMEOUT_MS: u64 = 2500;

impl Db {
  pub async fn connect_from_env(logger: Logger) -> Result<Self, AppError> {
    Self::connect(logger.clone(), dotenv!("DATABASE_URL")).await
  }

  async fn connect(logger: Logger, url: &'_ str) -> Result<Self, AppError> {
    info!(logger, "> Connecting to database... (timeout after {}ms)", DBCONN_TIMEOUT_MS);

    let pool = PgPoolOptions::new()
    .max_connections(30)
    .acquire_timeout(Duration::from_millis(DBCONN_TIMEOUT_MS))
    .connect(url)
    .await
    .map_err(|err| AppError::db_error(500, "CANNOT_CONNECT_TO_DB", Some(err.to_string())))?;

    Ok(Self {
      logger,
      pool
    })
  }

  /**
   * ! Beware of rwlock locks
   */
  pub fn get_writable(db_ref: &DbRef) -> Result<RwLockWriteGuard<'_, Db>, AppError> {
    let w = db_ref.write()
    .map_err(|err| AppError::db_error(500, "CANNOT_WRITE_DB", Some(err.to_string())))?;

    Ok(w)
  }

  /**
   * ! Beware of rwlock locks
   */
  pub fn get_readable(db_ref: &DbRef) -> Result<RwLockReadGuard<'_, Db>, AppError> {
    let r = db_ref.read()
    .map_err(|err| AppError::db_error(500, "CANNOT_READ_DB", Some(err.to_string())))?;

    Ok(r)
  }
}