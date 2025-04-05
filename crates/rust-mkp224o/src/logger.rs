
pub struct Logger;
impl Logger {
    pub fn init_with_default_level<S: Into<String>>(level: S) {
        Self::transfer_level_if_present();
        Self::set_default(level);

        env_logger::Builder::from_default_env()
            .format_timestamp(None)
            .format_target(false)
            .init();
    }

    fn transfer_level_if_present() {
        if let Ok(alternative_log_env) = std::env::var("LOG") {
            std::env::set_var("RUST_LOG", alternative_log_env);
        }
    }

    fn set_default<S: Into<String>>(default_log_level: S) {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", default_log_level.into());
        }
    }
}
