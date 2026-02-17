use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserId(pub u64);

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.discriminator == "0" {
            write!(f, "{}", self.username)
        } else {
            write!(f, "{}#{}", self.username, self.discriminator)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_display() {
        let id = UserId(12345);
        assert_eq!(format!("{}", id), "12345");
    }

    #[test]
    fn test_user_id_from_u64() {
        let id: UserId = 12345.into();
        assert_eq!(id, UserId(12345));
    }

    #[test]
    fn test_user_display_new_style() {
        let user = User {
            id: UserId(1),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            discriminator: "0".to_string(),
            global_name: None,
        };
        assert_eq!(format!("{}", user), "testuser");
    }

    #[test]
    fn test_user_display_old_style() {
        let user = User {
            id: UserId(1),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            discriminator: "1234".to_string(),
            global_name: None,
        };
        assert_eq!(format!("{}", user), "testuser#1234");
    }
}
