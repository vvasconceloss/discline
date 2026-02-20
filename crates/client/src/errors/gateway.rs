use thiserror::Error;
use tokio_tungstenite::tungstenite;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tungstenite::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Unexpected gateway closure")]
    UnexpectedClose,

    #[error("Gateway protocol error: {0}")]
    ProtocolError(String),

    #[error("Reconnect required")]
    ReconnectRequired,
}
