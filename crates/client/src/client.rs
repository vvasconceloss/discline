use rand::RngExt;
use reqwest::{
    Client, Method, Response, StatusCode,
    header::{AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT},
};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tokio::time::sleep;

use crate::errors::client::ClientError;

#[derive(Debug, Clone)]
pub struct HttpClient {
    http: Client,
    token: String,
    base_url: String,
}

impl HttpClient {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_BACKOFF_MS: u64 = 1000;

    const BASE_URL: &str = "https://discord.com/api/v10";
    const USER_AGENT_VALUE: &str = "DiscordBot (https://github.com/ovasconcelos/discline, 0.1.0)";

    pub fn new(token: String) -> Self {
        let mut headers = HeaderMap::new();
        let auth_token = format!("Bot {}", token);

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_token).expect("Invalid token format"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(Self::USER_AGENT_VALUE));

        let http = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http,
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

    pub fn set_base_url(&mut self, url: String) {
        self.base_url = url;
    }

    pub async fn wait(&self, attempt: u32) {
        let backoff = Self::INITIAL_BACKOFF_MS * 2u64.pow(attempt);
        let jitter = rand::rng().random_range(0..100);

        sleep(Duration::from_millis(backoff + jitter)).await;
    }

    pub async fn handle_error_status<T: DeserializeOwned>(
        &self,
        status: StatusCode,
        response: Response,
    ) -> Result<T, ClientError> {
        match status {
            StatusCode::UNAUTHORIZED => Err(ClientError::Unauthorized),
            StatusCode::FORBIDDEN => Err(ClientError::Forbidden),
            StatusCode::NOT_FOUND => Err(ClientError::NotFound {
                resource_type: "Resource".to_string(),
                resource_id: "Unknown".to_string(),
            }),
            StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(1);

                Err(ClientError::RateLimited { retry_after })
            }
            _ => {
                let error_message = response.text().await.unwrap_or_default();
                Err(ClientError::ApiError {
                    status: status.as_u16(),
                    message: error_message,
                })
            }
        }
    }

    pub async fn request<
        T: DeserializeOwned,
        B: serde::Serialize + Clone + std::marker::Copy,
        Q: serde::Serialize,
    >(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> Result<T, ClientError> {
        let url = format!("{}/{}", self.base_url(), endpoint.trim_start_matches('/'));

        for attempt in 0..Self::MAX_RETRIES {
            let mut request = self.http().request(method.clone(), &url);

            if let Some(b) = body {
                request = request.json(&b);
            }

            if let Some(q) = &query {
                request = request.query(&q);
            }

            let response = match request.send().await {
                Ok(res) => res,
                Err(_e) if attempt < Self::MAX_RETRIES - 1 => {
                    self.wait(attempt).await;
                    continue;
                }
                Err(e) => return Err(ClientError::Network(e)),
            };

            let status = response.status();

            if status.is_success() {
                return response
                    .json::<T>()
                    .await
                    .map_err(|e| ClientError::ParseError(e.to_string()));
            }

            if status.is_server_error() && attempt < Self::MAX_RETRIES - 1 {
                self.wait(attempt).await;
                continue;
            }

            return self.handle_error_status(status, response).await;
        }

        Err(ClientError::ApiError {
            status: 500,
            message: "Max retries reached".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_new() {
        let token = "test-token".to_string();
        let client = HttpClient::new(token.clone());

        assert_eq!(client.token(), &token);
        assert_eq!(client.base_url(), HttpClient::BASE_URL);
    }
}
