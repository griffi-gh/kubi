pub fn log_error(error: anyhow::Error) {
  log::error!("{}", error);
}
