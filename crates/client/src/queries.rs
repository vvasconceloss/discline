use serde::Serialize;
use types::message::MessageId;

#[derive(Debug, Serialize, Default, Clone, Copy)]
pub struct GetMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub around: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

impl GetMessagesQuery {
    pub fn with_limit(limit: u8) -> Self {
        Self {
            limit: Some(limit),
            ..Default::default()
        }
    }
}
