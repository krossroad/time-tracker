# Time Tracker

A macOS menu bar application that helps you track how you spend your time by periodically prompting you to categorize your activities.

## Features

- **Background Timer** - Runs quietly in your menu bar with configurable prompt intervals
- **Activity Categories** - Categorize time into deep work, meetings, email, admin, breaks, personal, or away
- **Idle Detection** - Automatically tracks "away" time when you're inactive
- **Calendar View** - Visual daily summaries and timelines of your activities
- **Missed Prompt Recovery** - Fill in entries retroactively if you miss a prompt

## Screenshots

<!-- Add screenshots here -->

## Tech Stack

**Frontend:** React 19, TypeScript, Vite, Zustand, Recharts

**Backend:** Tauri 2 (Rust), SQLite (rusqlite), Tokio

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/weekly-tracker.git
cd weekly-tracker

# Install dependencies
npm install
```

### Development

```bash
# Start the development server
npm run dev
```

### Build

```bash
# Build the production app
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Configuration

Settings are accessible from the app's tray menu:

| Setting | Default | Description |
|---------|---------|-------------|
| Interval | 15 min | How often you're prompted to log time |
| Idle Threshold | 5 min | Minutes of inactivity before marking as "away" |

## Data Storage

Your data is stored locally in SQLite at:
```
~/Library/Application Support/com.timetracker.app/time_tracker.db
```

## Project Structure

```
├── src/                    # Frontend (React/TypeScript)
│   ├── components/         # UI components
│   ├── services/           # Tauri API wrappers
│   ├── stores/             # Zustand state management
│   └── types/              # TypeScript definitions
├── src-tauri/              # Backend (Rust)
│   └── src/
│       ├── commands/       # Tauri command handlers
│       ├── db/             # Database layer
│       └── services/       # Background services
```

## License

MIT
