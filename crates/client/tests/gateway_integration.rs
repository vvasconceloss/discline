use client::gateway::{Event, Gateway};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message as WsMessage;

#[tokio::test]
async fn test_gateway_connect_and_ready() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let url = format!("ws://{}", addr);

    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws_server = accept_async(stream).await.unwrap();

        let hello = json!({
            "op": 10,
            "d": {
                "heartbeat_interval": 50 // 50ms
            }
        });
        ws_server
            .send(WsMessage::Text(hello.to_string().into()))
            .await
            .unwrap();

        let msg = ws_server.next().await.unwrap().unwrap();
        let identify: serde_json::Value = serde_json::from_str(msg.to_text().unwrap()).unwrap();
        assert_eq!(identify["op"], 2);
        assert_eq!(identify["d"]["token"], "test-token");

        let ready = json!({
            "op": 0,
            "t": "READY",
            "s": 1,
            "d": {
                "user": {
                    "id": 1,
                    "username": "testuser",
                    "discriminator": "0000",
                    "email": "test@test.com"
                },
                "guilds": [],
                "session_id": "test-session"
            }
        });
        ws_server
            .send(WsMessage::Text(ready.to_string().into()))
            .await
            .unwrap();

        let msg = ws_server.next().await.unwrap().unwrap();
        let heartbeat: serde_json::Value = serde_json::from_str(msg.to_text().unwrap()).unwrap();
        assert_eq!(heartbeat["op"], 1);

        let ack = json!({ "op": 11 });
        ws_server
            .send(WsMessage::Text(ack.to_string().into()))
            .await
            .unwrap();
    });

    let mut gateway = Gateway::connect_with_url("test-token".to_string(), &url)
        .await
        .expect("Failed to connect");

    let event = gateway
        .next_event()
        .await
        .expect("Failed to get READY event");
    match event {
        Event::Ready {
            user, session_id, ..
        } => {
            assert_eq!(user.username, "testuser");
            assert_eq!(session_id, "test-session");
        }
        _ => panic!("Expected READY event, got {:?}", event),
    }

    tokio::time::sleep(Duration::from_millis(100)).await;

    server_handle.await.unwrap();
}
