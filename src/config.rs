pub trait FromEnv {
  type Item;
  
  fn from_env(logger: slog::Logger) -> Self::Item;
}
