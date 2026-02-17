use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChannelId(pub u64);

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for ChannelId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_id_display() {
        let id = ChannelId(12345);
        assert_eq!(format!("{}", id), "12345");
    }

    #[test]
    fn test_channel_id_from_u64() {
        let id: ChannelId = 12345.into();
        assert_eq!(id, ChannelId(12345));
    }

    #[test]
    fn test_channel_display() {
        let channel = Channel {
            id: ChannelId(1),
            name: "general".to_string(),
        };
        assert_eq!(format!("{}", channel), "#general");
    }
}
