#[derive(Clone)]
pub struct Credentials {
    api_key: String,
    api_secret: String,
}

impl Credentials {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub(crate) fn api_secret(&self) -> &str {
        &self.api_secret
    }

    pub fn api_secret_masked(&self) -> &str {
        let secret = &self.api_secret;
        let len = secret.len();
        let start = &secret[..4];
        let end = &secret[len - 4..];
        let masked = "*".repeat(len - 8);
        Box::leak(format!("{}{}{}", start, masked, end).into_boxed_str())
    }
}
