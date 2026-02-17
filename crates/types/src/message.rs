use chrono::{DateTime, Utc};

use crate::{channel::ChannelId, user::User};

#[derive(Clone, Debug)]
pub struct MessageId(pub u64);

#[derive(Clone, Debug)]
pub struct Message {
    pub id: MessageId,
    pub author: User,
    pub content: String,
    pub channel_id: ChannelId,
    pub timestamp: DateTime<Utc>,
}
