const DEFAULT_BASE_PATH: &str = "https://api.cdp.coinbase.com/platform";

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key_id: String,
    pub api_key_secret: String,
    pub base_path: String,
}

impl Config {
    pub fn new(
        api_key_id: impl Into<String>,
        api_key_secret: impl Into<String>
    ) -> Self {
        Self {
            api_key_id: api_key_id.into(),
            api_key_secret: api_key_secret.into(),
            base_path: DEFAULT_BASE_PATH.to_string()
        }
    }

    pub fn with_base_path(self, base_path: impl Into<String>) -> Self {
        Self { base_path: base_path.into(), ..self }
    }
}

