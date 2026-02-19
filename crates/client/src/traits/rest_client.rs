use reqwest::Method;
use std::future::Future;
use types::{
    channel::{Channel, ChannelId},
    guild::GuildId,
    message::Message,
};

use crate::{client::HttpClient, errors::client::ClientError, queries::GetMessagesQuery};

pub trait RestClient {
    fn get_channels(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Channel>, ClientError>> + Send;
    fn get_messages(
        &self,
        channel_id: ChannelId,
        query: GetMessagesQuery,
    ) -> impl Future<Output = Result<Vec<Message>, ClientError>> + Send;
    fn send_message(
        &self,
        channel_id: ChannelId,
        content: &str,
    ) -> impl Future<Output = Result<Message, ClientError>> + Send;
}

impl RestClient for HttpClient {
    async fn get_channels(&self, guild_id: GuildId) -> Result<Vec<Channel>, ClientError> {
        let endpoint = format!("/guilds/{}/channels", guild_id);
        self.request(Method::GET, &endpoint, None::<()>, None::<()>)
            .await
    }

    async fn get_messages(
        &self,
        channel_id: ChannelId,
        query: GetMessagesQuery,
    ) -> Result<Vec<Message>, ClientError> {
        let endpoint = format!("/channels/{}/messages", channel_id);
        self.request(Method::GET, &endpoint, None::<()>, Some(query))
            .await
    }

    async fn send_message(
        &self,
        channel_id: ChannelId,
        content: &str,
    ) -> Result<Message, ClientError> {
        let endpoint = format!("/channels/{}/messages", channel_id);

        #[derive(serde::Serialize, Clone, Copy)]
        struct SendMessageBody<'a> {
            content: &'a str,
        }

        if content.len() > 2000 {
            return Err(ClientError::MessageTooLong);
        }

        self.request(
            Method::POST,
            &endpoint,
            Some(SendMessageBody { content }),
            None::<()>,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_unauthorized() {
        let client = HttpClient::new("invalid-token".to_string());
        let result = client.get_channels(GuildId(123)).await;

        match result {
            Err(ClientError::Unauthorized) => (),
            other => panic!("Expected Unauthorized error, got {:?}", other),
        }
    }
}
