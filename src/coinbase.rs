use std::vec;

use reqwest::Client;
use time::OffsetDateTime;

use crate::{config::Config, error::CoinbaseError};

#[derive(Debug, Clone)]
pub struct Coinbase {
    config: Config,
    client: Client,
}

impl Coinbase {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub fn generate_jwt(
        &self,
        request_method: Option<&str>,
        request_host: Option<&str>,
        request_path: Option<&str>,
        expires_in: Option<u64>,
        audience: Option<Vec<String>>,
    ) -> Result<String, CoinbaseError> {
        self.config.validate()?;

        let all_some = request_method.is_some() && request_host.is_some() && request_path.is_some();

        let all_none = request_method.is_none() && request_host.is_none() && request_path.is_none();

        if !(all_some || all_none) {
            return Err(CoinbaseError::InvalidRequestBindingParams);
        }

        let uris = Some(vec![format!(
            "{:?} {:?}{:?}",
            request_method, request_host, request_path
        )]);

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let expires = expires_in.unwrap_or(120);
        let exp = now + expires as i64;

        let aud = audience.unwrap_or_else(|| vec!["cdp_service".to_string()]);

        Ok("<not signed yet>".to_string())
    }

    pub fn build_url(&self, path: &str) -> String {
        self.config.build_url(path)
    }
}

#[cfg(test)]
mod tests {
    use super::Coinbase;
    use crate::config::Config;

    #[test]
    fn build_url_test() {
        let config = Config::new("sk-23fsd", "klds32");
        let coinbase = Coinbase::new(config);
        
        assert_eq!(
            coinbase.build_url("/v1/wallets"),
            "https://api.cdp.coinbase.com/platform/v1/wallets"
        );
    }
}
