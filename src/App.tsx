import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useAppStore } from "./stores/appStore";
import { CalendarView } from "./components/calendar/CalendarView";
import { PromptDialog } from "./components/tracker/PromptDialog";
import { SettingsPanel } from "./components/settings/SettingsPanel";
import "./App.css";

function App() {
  const { currentView, setCurrentView, setPendingTimestamp, loadSettings } =
    useAppStore();

  useEffect(() => {
    loadSettings();

    // Listen for prompt events from the timer
    const unlistenPrompt = listen<{ timestamp: number }>(
      "prompt-time-entry",
      (event) => {
        setPendingTimestamp(event.payload.timestamp);
      }
    );

    // Listen for return from away events
    const unlistenAway = listen<{ away_start: number; away_end: number }>(
      "return-from-away",
      (event) => {
        console.log("Returned from away:", event.payload);
        // Could show a special UI for filling in away time
      }
    );

    return () => {
      unlistenPrompt.then((unlisten) => unlisten());
      unlistenAway.then((unlisten) => unlisten());
    };
  }, [loadSettings, setPendingTimestamp]);

  return (
    <div className="app">
      <header className="app-header">
        <h1>Weekly Tracker</h1>
        <nav className="app-nav">
          <button
            className={currentView === "calendar" ? "active" : ""}
            onClick={() => setCurrentView("calendar")}
          >
            Calendar
          </button>
          <button
            className={currentView === "settings" ? "active" : ""}
            onClick={() => setCurrentView("settings")}
          >
            Settings
          </button>
        </nav>
      </header>

      <main className="app-main">
        {currentView === "prompt" && <PromptDialog />}
        {currentView === "calendar" && <CalendarView />}
        {currentView === "settings" && <SettingsPanel />}
      </main>
    </div>
  );
}

export default App;
