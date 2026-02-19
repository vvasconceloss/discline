use reqwest::{
    Client, Method, StatusCode,
    header::{AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT},
};
use serde::de::DeserializeOwned;

use crate::errors::client::ClientError;

#[derive(Debug, Clone)]
pub struct HttpClient {
    http: Client,
    token: String,
    base_url: String,
}

impl HttpClient {
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

    pub async fn request<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<B>,
    ) -> Result<T, ClientError> {
        let url = format!("{}/{}", self.base_url(), endpoint.trim_start_matches('/'));
        let mut request = self.http().request(method, &url);

        if let Some(b) = body {
            request = request.json(&b);
        }

        let response = request.send().await?;
        let status = response.status();

        match status {
            StatusCode::OK | StatusCode::CREATED => response
                .json::<T>()
                .await
                .map_err(|e| ClientError::ParseError(e.to_string())),
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
