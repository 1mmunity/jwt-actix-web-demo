use std::sync::{RwLock, Arc};

use actix_cors::Cors;
use actix_web::{HttpServer, App, web};
use slog::Logger;

use crate::{config::FromEnv, errors::AppError, routes, db::DbRef, RefreshTokenStore, utils::logger::LoggerRef};

pub struct Server {
  logger: Logger,
  addr: String
}

impl Server {
  pub async fn start(&self, db: DbRef, rts: RefreshTokenStore) -> Result<(), AppError> {
    info!(self.logger, "> Starting server...");
    info!(self.logger, "> http://{}", self.addr);
    let logger = self.logger.clone();
    let logger_ref: LoggerRef = Arc::new(RwLock::new(logger.clone()));
    HttpServer::new(move || {
      let cors = Cors::default()
      .allow_any_header()
      .allow_any_method()
      .allow_any_origin()
      .supports_credentials();

      App::new()
      .wrap(cors)
      .app_data(web::Data::new(rts.clone()))
      .app_data(web::Data::new(db.clone()))
      .app_data(web::Data::new(logger_ref.clone()))
      .service(routes::home::home)
      .service(routes::auth::auth_scope())
    })
    .bind(self.addr.clone())?
    .run()
    .await
    .map_err(|err| AppError::from(err))
  }

  /**
  Verbosely starts the server with configs given from env
   */
  pub async fn start_from_env(db: DbRef, logger: Logger, rts: RefreshTokenStore) -> Result<(), AppError> {
    let srv = Self::from_env(logger);
    srv.start(db, rts).await
  }
}

impl FromEnv for Server {
  type Item = Self;
  fn from_env(logger: Logger) -> Self {
    Self {
      logger,
      addr: dotenv!("HOST_ADDR").into()
    }
  }
}