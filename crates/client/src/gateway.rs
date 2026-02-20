use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::errors::gateway::GatewayError;
use types::message::Message;
use types::user::User;

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

#[derive(Debug, Serialize)]
pub struct IdentifyPayload {
    pub op: u8,
    pub d: IdentifyData,
}

#[derive(Debug, Serialize)]
pub struct IdentifyData {
    pub token: String,
    pub properties: IdentifyProperties,
    pub intents: u32,
}

#[derive(Debug, Serialize)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Debug, Deserialize)]
pub struct ReadyData {
    pub user: User,
    pub guilds: Vec<Value>,
    pub session_id: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    Ready {
        user: User,
        guilds: Vec<Value>,
        session_id: String,
    },
    MessageCreate(Message),
}

pub struct Gateway {
    pub ws_stream: futures_util::stream::SplitStream<WsStream>,
    pub ws_sink: Arc<Mutex<futures_util::stream::SplitSink<WsStream, WsMessage>>>,
    pub heartbeat_interval: Duration,
    pub last_sequence: Arc<Mutex<Option<u64>>>,
    pub token: String,
    pub session_id: Option<String>,
}

impl Gateway {
    pub const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

    pub async fn connect(token: String) -> Result<Self, GatewayError> {
        Self::connect_with_url(token, Self::GATEWAY_URL).await
    }

    pub async fn connect_with_url(token: String, url: &str) -> Result<Self, GatewayError> {
        let (ws, _) = connect_async(url).await?;
        let (mut sink, mut stream) = ws.split();

        let hello_msg = stream.next().await.ok_or(GatewayError::UnexpectedClose)??;
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

        let identify = IdentifyPayload {
            op: 2,
            d: IdentifyData {
                token: token.clone(),
                properties: IdentifyProperties {
                    os: std::env::consts::OS.to_string(),
                    browser: "discline".to_string(),
                    device: "discline".to_string(),
                },
                intents: 32767,
            },
        };

        sink.send(WsMessage::Text(serde_json::to_string(&identify)?.into()))
            .await?;

        let last_sequence = Arc::new(Mutex::new(None));
        let ws_sink = Arc::new(Mutex::new(sink));

        let heartbeat_sink = Arc::clone(&ws_sink);
        let heartbeat_seq = Arc::clone(&last_sequence);
        let interval = heartbeat_interval;

        tokio::spawn(async move {
            let mut timer = tokio::time::interval(interval);

            timer.tick().await;

            loop {
                timer.tick().await;

                let seq = *heartbeat_seq.lock().await;

                let heartbeat_payload = serde_json::json!({
                    "op": 1,
                    "d": seq
                });

                let mut sink = heartbeat_sink.lock().await;

                if let Err(e) = sink
                    .send(WsMessage::Text(heartbeat_payload.to_string().into()))
                    .await
                {
                    eprintln!("Failed to send heartbeat: {:?}", e);
                    break;
                }
            }
        });

        Ok(Self {
            ws_stream: stream,
            ws_sink,
            heartbeat_interval,
            last_sequence,
            token,
            session_id: None,
        })
    }

    pub async fn next_event(&mut self) -> Result<Event, GatewayError> {
        while let Some(msg) = self.ws_stream.next().await {
            let msg = msg?;

            if msg.is_close() {
                return Err(GatewayError::UnexpectedClose);
            }

            if !msg.is_text() {
                continue;
            }

            let payload: GatewayPayload = serde_json::from_str(msg.to_text().unwrap())?;

            if let Some(s) = payload.s {
                *self.last_sequence.lock().await = Some(s);
            }

            match payload.op {
                0 => {
                    if let Some(event_name) = payload.t.as_deref() {
                        match event_name {
                            "READY" => {
                                let ready_data: ReadyData = serde_json::from_value(payload.d)?;

                                self.session_id = Some(ready_data.session_id.clone());

                                return Ok(Event::Ready {
                                    user: ready_data.user,
                                    guilds: ready_data.guilds,
                                    session_id: ready_data.session_id,
                                });
                            }
                            "MESSAGE_CREATE" => {
                                let message: Message = serde_json::from_value(payload.d)?;

                                return Ok(Event::MessageCreate(message));
                            }
                            _ => continue,
                        }
                    }
                }
                1 => {
                    let seq = *self.last_sequence.lock().await;

                    let heartbeat_payload = serde_json::json!({
                        "op": 1,
                        "d": seq
                    });

                    let mut sink = self.ws_sink.lock().await;

                    sink.send(WsMessage::Text(heartbeat_payload.to_string().into()))
                        .await?;
                }
                11 => {
                    continue;
                }
                7 => {
                    return Err(GatewayError::ReconnectRequired);
                }
                9 => {
                    return Err(GatewayError::AuthenticationFailed);
                }
                _ => continue,
            }
        }

        Err(GatewayError::UnexpectedClose)
    }

    pub fn heartbeat_interval(&self) -> Duration {
        self.heartbeat_interval
    }
}
