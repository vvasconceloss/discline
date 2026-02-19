use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::errors::gateway::GatewayError;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Value,
    pub s: Option<u64>,
    pub t: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HelloData {
    pub heartbeat_interval: u64,
}

pub struct Gateway {
    pub ws: WsStream,
    pub heartbeat_interval: Duration,
    pub last_sequence: Option<u64>,
    pub token: String,
}

impl Gateway {
    pub const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

    pub async fn connect(token: String) -> Result<Self, GatewayError> {
        let (mut ws, _) = connect_async(Self::GATEWAY_URL).await?;

        let hello_msg = ws.next().await.ok_or(GatewayError::UnexpectedClose)??;

        let payload: GatewayPayload = serde_json::from_str(
            hello_msg
                .to_text()
                .map_err(|_| GatewayError::ProtocolError("Invalid UTF-8".into()))?,
        )?;

        if payload.op != 10 {
            return Err(GatewayError::ProtocolError(format!(
                "Expected HELLO (10), got {}",
                payload.op
            )));
        }

        let hello_data: HelloData = serde_json::from_value(payload.d)?;
        let heartbeat_interval = Duration::from_millis(hello_data.heartbeat_interval);

        Ok(Self {
            ws,
            heartbeat_interval,
            last_sequence: None,
            token,
        })
    }

    pub fn heartbeat_interval(&self) -> Duration {
        self.heartbeat_interval
    }
}
