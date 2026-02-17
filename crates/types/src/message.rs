use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{channel::ChannelId, user::User};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageId(pub u64);

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for MessageId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub author: User,
    pub content: String,
    pub channel_id: ChannelId,
    pub timestamp: DateTime<Utc>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.author,
            self.content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::UserId;
    use chrono::TimeZone;

    #[test]
    fn test_message_id_display() {
        let id = MessageId(12345);
        assert_eq!(format!("{}", id), "12345");
    }

    #[test]
    fn test_message_id_from_u64() {
        let id: MessageId = 12345.into();
        assert_eq!(id, MessageId(12345));
    }

    #[test]
    fn test_message_display() {
        let user = User {
            id: UserId(1),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            discriminator: "0".to_string(),
            global_name: None,
        };
        let timestamp = Utc.with_ymd_and_hms(2026, 2, 17, 12, 0, 0).unwrap();
        let message = Message {
            id: MessageId(1),
            author: user,
            content: "Hello, world!".to_string(),
            channel_id: ChannelId(1),
            timestamp,
        };
        assert_eq!(
            format!("{}", message),
            "[2026-02-17 12:00:00] testuser: Hello, world!"
        );
    }
}
