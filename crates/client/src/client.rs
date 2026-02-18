use reqwest::Client;
use types::{
    channel::{Channel, ChannelId},
    guild::GuildId,
    message::Message,
};

use crate::errors::client::ClientError;

#[derive(Debug)]
pub struct HttpClient {
    http: Client,
    token: String,
    base_url: String,
}

impl HttpClient {
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

pub trait RestClient {
    fn get_channels(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Channel>, ClientError>> + Send;
    fn get_messages(
        &self,
        channel_id: ChannelId,
        limit: u8,
    ) -> impl Future<Output = Result<Vec<Message>, ClientError>> + Send;
    fn send_message(
        &self,
        channel_id: ChannelId,
        content: &str,
    ) -> impl Future<Output = Result<Message, ClientError>> + Send;
}
