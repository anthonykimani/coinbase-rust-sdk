use reqwest::Client;

use crate::{config::Config, types::{Assets, NetworkIdentifier}};

pub struct Coinbase {
    pub config: Config,
    pub client: Client,
    pub networks: NetworkIdentifier,
    pub assets: Assets
}