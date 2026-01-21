use reqwest::Client;

use crate::{config::Config, error::CoinbaseError};

pub struct Coinbase {
    pub config: Config,
    pub client: Client,
}

impl Coinbase {
    pub fn generate_jwt(&self, request_method: Option<&str>, request_host: Option<&str>, request_path: Option<&str>, expires_in: Option<u64>, audience: Option<Vec<String>>) -> Result<String, CoinbaseError> {

    }
}