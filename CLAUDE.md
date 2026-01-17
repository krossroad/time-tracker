# Time Tracker - Project Documentation

## Project Overview

Time Tracker is a macOS menu bar application that helps users track their time by periodically prompting them to categorize their activities. The app runs in the background, shows a prompt at configurable intervals, and provides calendar views with daily summaries and timelines.

**Purpose:** Enable users to understand how they spend their time throughout the workday by categorizing activities into predefined buckets (deep work, meetings, email, admin, breaks, personal, away).

**Key Features:**
- Background timer with configurable intervals
- Automatic idle detection and "away" tracking
- Time entry prompts with category selection
- Calendar view with daily summaries and timelines
- Missed prompt tracking and retroactive entry filling
- macOS menu bar integration (no dock icon)

## Tech Stack

### Frontend
- **Framework:** React 19.1.0 with TypeScript 5.8
- **Build Tool:** Vite 7
- **State Management:** Zustand 5.0 (single global store pattern)
- **UI Libraries:** Recharts 3.6 (charts), date-fns 4.1 (date handling)
- **Desktop Integration:** Tauri API 2

### Backend
- **Runtime:** Tauri 2 (Rust-based)
- **Database:** rusqlite 0.31 (SQLite with bundled driver)
- **Async Runtime:** tokio 1 with full features
- **Serialization:** serde 1 with derive macros
- **Date/Time:** chrono 0.4

## Project Structure

```
time-tracker/
├── src/                          # Frontend React/TypeScript code
│   ├── components/              # React components
│   │   ├── calendar/           # Calendar view components
│   │   ├── settings/           # Settings panel
│   │   └── tracker/            # Time entry prompts and UI
│   ├── services/               # API layer (Tauri invoke wrappers)
│   ├── stores/                 # Zustand state management
│   ├── types/                  # TypeScript type definitions
│   ├── App.tsx                 # Root component
│   └── main.tsx                # React entry point
├── src-tauri/                   # Backend Rust code
│   ├── src/
│   │   ├── commands/           # Tauri command handlers (RPC endpoints)
│   │   ├── db/                 # Database layer
│   │   │   ├── connection.rs   # DB initialization and path setup
│   │   │   ├── migrations.rs   # Schema migrations
│   │   │   ├── models.rs       # Data models and enums
│   │   │   └── repositories/   # Repository pattern for data access
│   │   │       ├── mod.rs      # Exports and helper functions
│   │   │       ├── error.rs    # RepositoryError enum
│   │   │       ├── time_entry.rs    # TimeEntryRepository
│   │   │       ├── missed_prompt.rs # MissedPromptRepository
│   │   │       └── settings.rs      # SettingsRepository
│   │   ├── services/           # Background services
│   │   │   ├── idle_detector.rs # System idle detection
│   │   │   └── timer.rs        # Interval timer and prompt logic
│   │   ├── lib.rs              # Library entry with app setup
│   │   └── main.rs             # Binary entry point
│   └── Cargo.toml              # Rust dependencies
├── package.json                # Node.js dependencies
└── vite.config.ts              # Vite configuration

```

### Key Directories

- **`src/components/`** - React UI components organized by feature domain
- **`src/stores/`** - Zustand store with actions for data fetching and state updates
- **`src/services/`** - Frontend API layer wrapping Tauri `invoke()` calls
- **`src-tauri/src/commands/`** - Backend command handlers exposed to frontend
- **`src-tauri/src/db/`** - Database abstraction with models, migrations, and repositories
- **`src-tauri/src/db/repositories/`** - Repository pattern for centralized data access
- **`src-tauri/src/services/`** - Background services (timer, idle detection)

## Essential Commands

### Development
```bash
npm run dev          # Start Vite dev server + Tauri app in dev mode
npm run tauri dev    # Alternative: run Tauri directly
```

### Build
```bash
npm run build        # Build frontend (TypeScript compilation + Vite build)
npm run tauri build  # Build full desktop app bundle
```

### Preview
```bash
npm run preview      # Preview production build locally
```

## Database

- **Location:** `~/Library/Application Support/com.timetracker.app/time_tracker.db`
- **Tables:** `time_entries`, `missed_prompts`, `settings`
- **Migrations:** Auto-run on app startup (src-tauri/src/db/migrations.rs:8)

## Configuration

Settings stored in the `settings` table with key-value pairs:
- `interval_minutes` (default: 15) - Prompt interval
- `idle_threshold_minutes` (default: 5) - Minutes before marking as "away"

Access via `get_setting`, `set_setting`, `get_all_settings` commands.

## Important Implementation Details

### Timer Alignment
The background timer aligns to interval boundaries (e.g., :00, :15, :30, :45 for 15-min intervals) - see src-tauri/src/services/timer.rs:103

### Idle Detection
Uses platform-specific APIs to detect user inactivity. Auto-creates "away" entries when idle threshold is exceeded - see src-tauri/src/services/idle_detector.rs

### Event System
Tauri events communicate backend → frontend:
- `prompt-time-entry` - Triggers time entry prompt
- `return-from-away` - Notifies when user returns after idle period

Listeners setup in src/App.tsx:17-36

### State Flow
1. Tauri commands (Rust) ↔ API service (TypeScript) via `invoke()`
2. API service ↔ Zustand store actions
3. Zustand store ↔ React components via hooks

Reference: src/services/api.ts, src/stores/appStore.ts

## Additional Documentation

For detailed patterns and conventions, see:

- **[Architectural Patterns](./.claude/docs/architectural_patterns.md)** - Design patterns, state management, module organization, and frontend-backend communication conventions

## Key Files Reference

| Purpose | File Location |
|---------|--------------|
| App initialization | src-tauri/src/lib.rs:15 |
| Tray icon setup | src-tauri/src/lib.rs:32-69 |
| Timer service | src-tauri/src/services/timer.rs:14 |
| Time entry commands | src-tauri/src/commands/time_entry.rs |
| Database connection | src-tauri/src/db/connection.rs:10 |
| TimeEntryRepository | src-tauri/src/db/repositories/time_entry.rs |
| SettingsRepository | src-tauri/src/db/repositories/settings.rs |
| MissedPromptRepository | src-tauri/src/db/repositories/missed_prompt.rs |
| Type definitions | src/types/index.ts |
| Global state store | src/stores/appStore.ts:41 |
| Main view router | src/App.tsx:39-64 |
| Category constants | src/types/index.ts:33-41 |
