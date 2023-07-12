use std::sync::{Arc, RwLock};

use slog::Drain;

pub type LoggerRef = Arc<RwLock<slog::Logger>>;

pub fn configure_log() -> slog::Logger {
  let decorator = slog_term::TermDecorator::new().build();
  let drain = slog_term::FullFormat::new(decorator).build().fuse();
  let drain = slog_async::Async::new(drain).build().fuse();

  slog::Logger::root(drain, o!())
}