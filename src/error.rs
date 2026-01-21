
pub enum CoinbaseError {
    MissingApiKeyId,
    MissingApiKeySecret,
    InvalidRequestBindingParams,
    InvalidKeyFormat,
    Base64Decode,
    EcKeyParse,
    Ed25519KeyParse,
    JwtSign
}