use super::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }
}
