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
