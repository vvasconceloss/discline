use reqwest::Client;

#[derive(Debug)]
pub struct RestClient {
    http: Client,
    token: String,
    base_url: String,
}

impl RestClient {
    const BASE_URL: &str = "https://discord.com/api/v10";

    pub fn new(token: String) -> Self {
        Self {
            http: Client::new(),
            token,
            base_url: Self::BASE_URL.to_string(),
        }
    }

    pub fn http(&self) -> &Client {
        &self.http
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
