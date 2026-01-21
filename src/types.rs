use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum NetworkIdentifier {
    BaseSepolia,
    BaseMainnet,
    EthereumHolesky,
    EthereumSepolia,
    EthereumMainnet,
    PolygonMainnet,
    SolanaDevnet,
    SolanaMainnet,
    ArbitrumMainnet,
    ArbitrumSepolia,
    BitcoinMainnet,
    NearTestnet,
    NearMainnet,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Assets {
    Eth,
    Wei,
    Gwei,
    Usdc,
    Weth,
    Sol,
    Lamport,
    Eurc,
    Cbbtc,
}