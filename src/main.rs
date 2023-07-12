#![allow(dead_code, unused_variables)]

#[macro_use]
mod resp;
mod server;
mod utils;
mod config;
mod db;
mod errors;
mod routes;
mod models;
mod auth;
mod middlewares;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use db::{DbRef, redis::RefreshTokenStore};
use errors::AppError;
use std::sync::{RwLock, Arc};

#[tokio::main]
async fn main() -> Result<(), AppError> {
  let logger = utils::logger::configure_log();
  let db: DbRef = Arc::new(RwLock::new(db::Db::connect_from_env(logger.clone()).await?));
  let refresh_token_store: RefreshTokenStore = Arc::new(RwLock::new(db::redis::RefreshTokenRedis::init_from_env(logger.clone())?));

  server::Server::start_from_env(db.clone(), logger.clone(), refresh_token_store.clone()).await
}