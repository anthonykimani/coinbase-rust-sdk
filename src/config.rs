pub struct Config {
    pub api_key_id: String,
    pub api_key_secret: String,
    pub base_path: String,
}

impl Config {
    pub fn new(
        api_key_id: impl Into<String>,
        api_key_secret: impl Into<String>,
        base_path: impl Into<String>,
    ) -> Self {
        Self {
            api_key_id: api_key_id.into(),
            api_key_secret: api_key_secret.into(),
            base_path: base_path.into()
        }
    }
}
