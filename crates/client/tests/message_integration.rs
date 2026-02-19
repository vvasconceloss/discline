use chrono::Utc;
use client::client::HttpClient;
use client::errors::client::ClientError;
use client::traits::rest_client::RestClient;
use mockito::Server;
use types::channel::ChannelId;
use types::message::Message;
use types::user::User;

#[tokio::test]
async fn test_send_message_success() {
    let mut server = Server::new_async().await;
    let channel_id = ChannelId(123);
    let content = "Hello from test!";

    let mock_message = Message {
        id: 1.into(),
        author: User {
            id: 1.into(),
            username: "testuser".into(),
            discriminator: "0000".into(),
            global_name: None,
            email: "test@test.com".into(),
        },
        content: content.into(),
        channel_id,
        timestamp: Utc::now(),
    };

    let _m = server
        .mock("POST", "/channels/123/messages")
        .match_header("authorization", "Bot test-token")
        .match_body(mockito::Matcher::Json(
            serde_json::json!({ "content": content }),
        ))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_message).unwrap())
        .create_async()
        .await;

    let mut client = HttpClient::new("test-token".into());
    client.set_base_url(server.url());

    let result = client.send_message(channel_id, content).await;

    assert!(result.is_ok());
    let msg = result.unwrap();
    assert_eq!(msg.content, content);
    assert_eq!(msg.channel_id, channel_id);
}

#[tokio::test]
async fn test_send_message_too_long() {
    let content = "a".repeat(2001);
    let client = HttpClient::new("test-token".into());
    let channel_id = ChannelId(123);

    let result = client.send_message(channel_id, &content).await;

    match result {
        Err(ClientError::MessageTooLong) => (),
        _ => panic!("Expected MessageTooLong error"),
    }
}

#[tokio::test]
async fn test_send_message_retry_success() {
    let mut server = Server::new_async().await;
    let channel_id = ChannelId(123);
    let content = "Retry test";

    let mock_message = Message {
        id: 2.into(),
        author: User {
            id: 1.into(),
            username: "testuser".into(),
            discriminator: "0000".into(),
            global_name: None,
            email: "test@test.com".into(),
        },
        content: content.into(),
        channel_id,
        timestamp: Utc::now(),
    };

    let _m1 = server
        .mock("POST", "/channels/123/messages")
        .with_status(500)
        .expect(1)
        .create_async()
        .await;

    let _m2 = server
        .mock("POST", "/channels/123/messages")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_message).unwrap())
        .expect(1)
        .create_async()
        .await;

    let mut client = HttpClient::new("test-token".into());
    client.set_base_url(server.url());

    let result = client.send_message(channel_id, content).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, content);
}
