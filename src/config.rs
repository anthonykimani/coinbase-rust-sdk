pub struct Config {
    pub api_key_name: String,
    pub private_key: String,
    pub use_server_signer: bool,
}

impl Config {
    pub fn new(
        api_key_name: impl Into<String>,
        private_key: impl Into<String>,
        use_server_signer: bool,
    ) -> Self {
        Self {
            api_key_name: api_key_name.into(),
            private_key: private_key.into(),
            use_server_signer,
        }
    }
}
