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
