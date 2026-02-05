pub mod error;
pub mod missed_prompt;
pub mod settings;
pub mod time_entry;

pub use error::{RepositoryError, Result};
pub use missed_prompt::MissedPromptRepository;
pub use settings::SettingsRepository;
pub use time_entry::TimeEntryRepository;

/// Convert SQLite integer (0/1) to bool
pub fn int_to_bool(val: i32) -> bool {
    val != 0
}

/// Convert bool to SQLite integer (0/1)
pub fn bool_to_int(val: bool) -> i32 {
    if val { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_bool() {
        assert!(!int_to_bool(0));
        assert!(int_to_bool(1));
        assert!(int_to_bool(42));
        assert!(int_to_bool(-1));
    }

    #[test]
    fn test_bool_to_int() {
        assert_eq!(bool_to_int(true), 1);
        assert_eq!(bool_to_int(false), 0);
    }

    #[test]
    fn test_roundtrip() {
        assert!(int_to_bool(bool_to_int(true)));
        assert!(!int_to_bool(bool_to_int(false)));
    }
}
