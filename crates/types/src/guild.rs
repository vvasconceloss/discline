use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuildId(pub u64);

impl fmt::Display for GuildId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for GuildId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: GuildId,
    pub name: String,
    pub description: Option<String>,
}

impl fmt::Display for Guild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guild_id_display() {
        let id = GuildId(12345);
        assert_eq!(format!("{}", id), "12345");
    }

    #[test]
    fn test_guild_id_from_u64() {
        let id: GuildId = 12345.into();
        assert_eq!(id, GuildId(12345));
    }

    #[test]
    fn test_guild_display() {
        let guild = Guild {
            id: GuildId(1),
            name: "Test Guild".to_string(),
            description: None,
        };
        assert_eq!(format!("{}", guild), "Test Guild");
    }
}
