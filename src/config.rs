use crate::error::CoinbaseError;

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

    pub fn validate(&self) -> Result<(), CoinbaseError> {
        if self.api_key_id.trim().is_empty() {
            return Err(CoinbaseError::MissingApiKeyId)
        }

        if self.api_key_secret.trim().is_empty() {
            return Err(CoinbaseError::MissingApiKeySecret)
        }

        Ok(())
    }

    pub fn build_url(&self, path: &str) -> String {
        let base_path = self.base_path.trim_end_matches('/');
        let path_str = path.trim_start_matches('/');
        format!("{base_path}/{path_str}")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn build_url_handles_slashes_test() {
        // create a config with default base path
        let config = Config::new("id", "secret");

        // path with leading slash
        let build_url = config.build_url("/v1/wallets");
        assert_eq!(build_url, "https://api.cdp.coinbase.com/platform/v1/wallets");

        // config with trailing slash
        let config = Config::new("sk-####", "HSKD").with_base_path("https://api.cdp.coinbase.com/platform/");

        let url = config.build_url("v1/wallets");
        assert_eq!(url, "https://api.cdp.coinbase.com/platform/v1/wallets");
    }
}



