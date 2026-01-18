import { useState } from "react";
import { format } from "date-fns";
import { useAppStore } from "../../stores/appStore";
import { CategorySelector } from "./CategorySelector";
import { Category } from "../../types";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function PromptDialog() {
  const { pendingTimestamp, createEntry, setCurrentView, settings } = useAppStore();
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(
    null
  );
  const [notes, setNotes] = useState("");

  const handleSubmit = async () => {
    if (!selectedCategory || !pendingTimestamp || !notes.trim()) return;

    await createEntry(pendingTimestamp, selectedCategory, notes);
    setSelectedCategory(null);
    setNotes("");

    // Hide window after submitting
    try {
      const window = getCurrentWindow();
      await window.hide();
    } catch (e) {
      console.error("Failed to hide window:", e);
    }
  };

  const handleSkip = async () => {
    setCurrentView("calendar");
    setSelectedCategory(null);
    setNotes("");

    try {
      const window = getCurrentWindow();
      await window.hide();
    } catch (e) {
      console.error("Failed to hide window:", e);
    }
  };

  const intervalMinutes = parseInt(settings.interval_minutes || "15", 10);
  const intervalStart = pendingTimestamp
    ? format(new Date(pendingTimestamp * 1000), "h:mm a")
    : "";
  const intervalEnd = pendingTimestamp
    ? format(new Date((pendingTimestamp + intervalMinutes * 60) * 1000), "h:mm a")
    : "";
  const formattedInterval = pendingTimestamp ? `${intervalStart} - ${intervalEnd}` : "";

  return (
    <div className="prompt-dialog">
      <h2>What did you work on?</h2>
      <p className="prompt-time">{formattedInterval}</p>

      <CategorySelector
        selectedCategory={selectedCategory}
        onSelect={setSelectedCategory}
      />

      <div className="notes-section">
        <input
          type="text"
          placeholder="Notes (required)"
          value={notes}
          onChange={(e) => setNotes(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && selectedCategory && notes.trim()) {
              handleSubmit();
            }
          }}
          required
        />
      </div>

      <div className="prompt-actions">
        <button className="btn-secondary" onClick={handleSkip}>
          Skip
        </button>
        <button
          className="btn-primary"
          onClick={handleSubmit}
          disabled={!selectedCategory || !notes.trim()}
        >
          Save
        </button>
      </div>
    </div>
  );
}
