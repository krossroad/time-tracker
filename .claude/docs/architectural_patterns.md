# Architectural Patterns & Design Decisions

This document outlines recurring patterns and conventions used throughout the Time Tracker codebase.

## Frontend-Backend Communication Pattern

### Tauri Command Pattern
The application uses Tauri's command system as an RPC (Remote Procedure Call) layer between Rust backend and TypeScript frontend.

**Backend (Rust):**
- Commands defined with `#[tauri::command]` macro
- Registered in invoke handler - src-tauri/src/lib.rs:96-107
- Accept `State<'_, Database>` for database access
- Return `Result<T, String>` for error handling

Example: src-tauri/src/commands/time_entry.rs:4-31

**Frontend (TypeScript):**
- API service wraps `invoke()` calls with type safety
- Async functions mirror backend command signatures
- Parameter name conversion (camelCase ↔ snake_case handled by Tauri)

Example: src/services/api.ts:4-22

**Pattern Benefits:**
- Type-safe RPC with compile-time checks on both sides
- Clear API contract between frontend/backend
- Centralized error handling

## State Management Pattern

### Zustand Single Store
Uses a single global store with Zustand instead of Redux or Context API.

**Store Structure:**
- State: Data properties (entries, settings, UI state)
- Actions: Functions that modify state and trigger side effects
- Located in: src/stores/appStore.ts:41

**Key Conventions:**
- Actions are colocated with state in the store definition
- Async operations (API calls) are handled within action functions
- Components access store via `useAppStore()` hook
- Fine-grained subscriptions possible (can select specific state slices)

**Data Flow:**
```
Component → Action → API Service → Tauri Command → Database
                ↓
         Update State
                ↓
      Re-render Component
```

Example: src/stores/appStore.ts:98-109 (createEntry action)

## Module Organization Pattern

### Barrel Exports (Re-export Pattern)
Both Rust and TypeScript use module index files to create clean public APIs.

**Rust modules:**
- `mod.rs` files re-export items from submodules
- Example: src-tauri/src/commands/mod.rs:1-5
- Pattern: `pub use module_name::*;`

**TypeScript modules:**
- `index.ts` files export types and functions
- Example: src/types/index.ts
- Pattern: Named exports from single file or re-exports

**Benefits:**
- Cleaner imports (`use commands::create_time_entry` vs `use commands::time_entry::create_time_entry`)
- Encapsulation of internal module structure
- Single source of truth for public API

## Database Access Pattern

### Thread-Safe Connection with Mutex
SQLite connection wrapped in Mutex for safe concurrent access.

**Pattern:**
- Database struct holds `Mutex<Connection>` - src-tauri/src/db/connection.rs:5-22
- Commands acquire lock with `.lock().map_err()`
- Lock automatically released when guard goes out of scope
- Repositories take ownership of MutexGuard for their lifetime

**Why this pattern:**
- SQLite doesn't support concurrent writes
- Mutex ensures only one command accesses DB at a time
- Tauri's async runtime requires Send/Sync types

## Repository Pattern

### Centralized Data Access Layer
All database operations are encapsulated in repository structs, separating data access from business logic.

**Repository Structure:**
```
src-tauri/src/db/repositories/
├── mod.rs           # Exports, helper functions (int_to_bool, bool_to_int)
├── error.rs         # RepositoryError enum
├── time_entry.rs    # TimeEntryRepository
├── missed_prompt.rs # MissedPromptRepository
└── settings.rs      # SettingsRepository
```

**Pattern:**
```rust
// Repository takes ownership of MutexGuard
pub struct TimeEntryRepository<'a> {
    conn: MutexGuard<'a, Connection>,
}

impl<'a> TimeEntryRepository<'a> {
    pub fn new(conn: MutexGuard<'a, Connection>) -> Self;
    pub fn create(...) -> Result<i64>;
    pub fn find_by_date_range(start: i64, end: i64) -> Result<Vec<TimeEntry>>;
    // ...
}
```

**Usage in Commands:**
```rust
#[tauri::command]
pub fn get_entries_for_date(db: State<'_, Database>, ...) -> Result<Vec<TimeEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let repo = TimeEntryRepository::new(conn);
    repo.find_by_date_range(start, end).map_err(Into::into)
}
```

**Available Repositories:**

| Repository | Key Methods |
|------------|-------------|
| `TimeEntryRepository` | `create`, `create_away_entry`, `find_by_date_range`, `find_raw_by_date_range`, `update_category`, `update_notes`, `delete` |
| `MissedPromptRepository` | `create`, `find_by_date_range`, `delete_by_timestamp` |
| `SettingsRepository` | `get`, `set`, `get_all`, `get_interval_minutes`, `get_idle_threshold_minutes`, `is_notification_enabled`, `get_notification_sound` |

**Typed Settings Getters:**
SettingsRepository provides typed convenience methods that handle parsing and defaults:
- `get_interval_minutes() -> u64` (default: 15)
- `get_idle_threshold_minutes() -> u32` (default: 5)
- `is_notification_enabled() -> bool` (default: true)
- `get_notification_sound() -> String` (default: "default")

**Helper Functions (mod.rs):**
- `int_to_bool(val: i32) -> bool` - Convert SQLite integer to bool
- `bool_to_int(val: bool) -> i32` - Convert bool to SQLite integer

**Error Handling:**
```rust
pub enum RepositoryError {
    DatabaseError(rusqlite::Error),
    LockError(String),
    NotFound(String),
    InvalidData(String),
}
// Implements From<RepositoryError> for String for command compatibility
```

**Benefits:**
- Centralized SQL queries in one location per entity
- Eliminates duplicated row mapping code
- Type-safe settings access with defaults
- Consistent error handling across all data operations
- Commands focus on business logic, not SQL
- Easier testing (can mock repositories)

**References:**
- TimeEntryRepository: src-tauri/src/db/repositories/time_entry.rs
- SettingsRepository: src-tauri/src/db/repositories/settings.rs
- Usage in commands: src-tauri/src/commands/time_entry.rs:14-15
- Usage in services: src-tauri/src/services/timer.rs:39-46

## Component Composition Pattern

### Feature-Based Organization
React components grouped by feature domain rather than technical role.

**Directory structure:**
```
components/
  ├── calendar/      # Calendar, timeline, summaries
  ├── settings/      # Settings panel
  └── tracker/       # Prompts, category selector
```

**Conventions:**
- Each feature folder contains related components
- Components within a feature can import from siblings
- Cross-feature imports should be minimal
- Shared utilities/types live in top-level directories

**Component patterns:**
- Hooks called at top of components
- Event handlers defined as arrow functions within component
- Zustand actions called directly from event handlers
- Props interface defined inline or via type imports

Example: src/components/tracker/PromptDialog.tsx:8-86

## Error Handling Pattern

### Result<T, String> Convention
Backend commands use Rust's `Result` type with String errors.

**Pattern:**
```rust
pub fn command_name(...) -> Result<ReturnType, String> {
    operation().map_err(|e| e.to_string())?;
    Ok(result)
}
```

**Frontend handling:**
- API calls are wrapped in try-catch
- Errors logged to console
- UI shows loading states during async operations
- Example: src/stores/appStore.ts:61-72

**Benefits:**
- Consistent error types across command boundary
- Frontend can display error messages directly
- Simple serialization for IPC

## Service Layer Pattern

### Background Services with Tokio
Long-running background tasks run in separate async tasks.

**Implementation:**
- Services spawned in app setup - src-tauri/src/lib.rs:90-92
- Use tokio channels for communication (mpsc)
- Services can emit Tauri events to frontend
- Example: src-tauri/src/services/timer.rs:13-101

**Timer Service Pattern:**
- Runs infinite loop with `tokio::select!`
- Listens for both interval ticks and command channel
- Emits events to trigger UI updates
- Commands sent via mpsc channel for timer control

**Benefits:**
- Non-blocking background operations
- Clean shutdown via channel commands
- Event-driven frontend updates

## Type Synchronization Pattern

### Mirrored Types Across Language Boundary
TypeScript and Rust types mirror each other for data models.

**Examples:**
- TimeEntry: src/types/index.ts:1-10 ↔ src-tauri/src/db/models.rs:3-13
- Category enum: src/types/index.ts:24-31 ↔ src-tauri/src/db/models.rs:29-65

**Conventions:**
- Rust types use serde derive for JSON serialization
- TypeScript types match serialized JSON structure
- Enums use snake_case string values for interop
- Category enum includes conversion methods (as_str, from_str)

**Maintenance:**
- Update both type definitions when modifying data models
- Ensure serde serialization format matches TS expectations
- Use optional types (Option<T> / null) consistently

## Timestamp Handling Pattern

### Unix Epoch Seconds
All timestamps stored and transmitted as seconds since Unix epoch.

**Convention:**
- Database stores `i64` timestamps
- TypeScript receives `number` timestamps
- date-fns converts to/from JavaScript Date objects
- Timer aligns timestamps to interval boundaries

**Example conversions:**
- Rust: `chrono::Local::now().timestamp()`
- TypeScript: `Math.floor(date.getTime() / 1000)`
- Display: `format(new Date(timestamp * 1000), "h:mm a")`

Reference: src/stores/appStore.ts:64-66, src-tauri/src/services/timer.rs:103-106

## Tauri State Management Pattern

### Managed State with Dependency Injection
Tauri's managed state provides dependency injection for commands.

**Pattern:**
```rust
// Setup phase
app.manage(database);

// Command usage
#[tauri::command]
pub fn command(db: State<'_, Database>) -> Result<T, String> {
    // Use db...
}
```

**Location:** src-tauri/src/lib.rs:30, command examples in src-tauri/src/commands/

**Benefits:**
- Single database instance shared across commands
- Type-safe dependency injection
- Automatic lifetime management by Tauri
- Thread-safe access via managed state

## Date Range Query Pattern

### Start/End Timestamp Parameters
Database queries accept inclusive start and exclusive end timestamps.

**Convention:**
```rust
WHERE timestamp >= start_timestamp AND timestamp < end_timestamp
```

**Frontend usage:**
- Calculate using date-fns: `startOfDay()`, `endOfDay()`
- Convert to Unix seconds before API call
- Pattern used consistently for entries and missed prompts

Example: src/stores/appStore.ts:61-66, src-tauri/src/commands/time_entry.rs:34-46
