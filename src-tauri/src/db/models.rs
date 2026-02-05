use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: Option<i64>,
    pub timestamp: i64,
    pub category: String,
    pub duration_minutes: i32,
    pub is_away: bool,
    pub is_retroactive: bool,
    pub notes: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissedPrompt {
    pub id: Option<i64>,
    pub timestamp: i64,
    pub reason: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Category {
    DeepWork,
    Meetings,
    Admin,
    Break,
    Away,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::DeepWork => "deep_work",
            Category::Meetings => "meetings",
            Category::Admin => "admin",
            Category::Break => "break",
            Category::Away => "away",
        }
    }

    pub fn from_str(s: &str) -> Option<Category> {
        match s {
            "deep_work" => Some(Category::DeepWork),
            "meetings" => Some(Category::Meetings),
            "admin" => Some(Category::Admin),
            "break" => Some(Category::Break),
            "away" => Some(Category::Away),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_as_str() {
        assert_eq!(Category::DeepWork.as_str(), "deep_work");
        assert_eq!(Category::Meetings.as_str(), "meetings");
        assert_eq!(Category::Admin.as_str(), "admin");
        assert_eq!(Category::Break.as_str(), "break");
        assert_eq!(Category::Away.as_str(), "away");
    }

    #[test]
    fn test_category_from_str_valid() {
        assert_eq!(Category::from_str("deep_work"), Some(Category::DeepWork));
        assert_eq!(Category::from_str("meetings"), Some(Category::Meetings));
        assert_eq!(Category::from_str("admin"), Some(Category::Admin));
        assert_eq!(Category::from_str("break"), Some(Category::Break));
        assert_eq!(Category::from_str("away"), Some(Category::Away));
    }

    #[test]
    fn test_category_from_str_invalid() {
        assert_eq!(Category::from_str("invalid"), None);
        assert_eq!(Category::from_str(""), None);
        assert_eq!(Category::from_str("Deep Work"), None);
        assert_eq!(Category::from_str("AWAY"), None);
    }

    #[test]
    fn test_category_roundtrip() {
        let variants = [
            Category::DeepWork,
            Category::Meetings,
            Category::Admin,
            Category::Break,
            Category::Away,
        ];
        for variant in variants {
            assert_eq!(Category::from_str(variant.as_str()), Some(variant));
        }
    }
}
