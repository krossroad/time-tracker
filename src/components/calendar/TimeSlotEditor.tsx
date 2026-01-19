import { useState, useEffect } from "react";
import { format } from "date-fns";
import { useAppStore } from "../../stores/appStore";
import { CategorySelector } from "../tracker/CategorySelector";
import { Category, TimeEntry, CATEGORIES } from "../../types";

interface TimeSlotEditorProps {
  timestamp: number;
  existingEntry: TimeEntry | null;
  onClose: () => void;
}

export function TimeSlotEditor({ timestamp, existingEntry, onClose }: TimeSlotEditorProps) {
  const { settings, createEntry, updateEntry, deleteEntry, loadEntriesForDate, selectedDate } = useAppStore();
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(
    existingEntry?.category || null
  );
  const [notes, setNotes] = useState(existingEntry?.notes || "");

  const intervalMinutes = parseInt(settings.interval_minutes || "15", 10);
  const intervalStart = format(new Date(timestamp * 1000), "h:mm a");
  const intervalEnd = format(new Date((timestamp + intervalMinutes * 60) * 1000), "h:mm a");

  useEffect(() => {
    if (existingEntry) {
      setSelectedCategory(existingEntry.category);
      setNotes(existingEntry.notes || "");
    }
  }, [existingEntry]);

  const handleSave = async () => {
    if (!selectedCategory || !notes.trim()) return;

    if (existingEntry && existingEntry.id) {
      await updateEntry(existingEntry.id, selectedCategory, notes);
    } else {
      await createEntry(timestamp, selectedCategory, notes, true);
    }
    await loadEntriesForDate(selectedDate);
    onClose();
  };

  const handleDelete = async () => {
    if (existingEntry && existingEntry.id) {
      await deleteEntry(existingEntry.id);
      await loadEntriesForDate(selectedDate);
    }
    onClose();
  };

  const handleOverlayClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  return (
    <div className="time-slot-editor-overlay" onClick={handleOverlayClick}>
      <div className="time-slot-editor">
        <div className="time-slot-editor-header">
          <h3>{existingEntry ? "Edit Time Slot" : "Fill Time Slot"}</h3>
          <button className="close-btn" onClick={onClose}>×</button>
        </div>

        <p className="time-slot-interval">{intervalStart} - {intervalEnd}</p>

        {existingEntry && (
          <div className="current-entry-info">
            <span
              className="current-category-badge"
              style={{ backgroundColor: CATEGORIES.find(c => c.value === existingEntry.category)?.color }}
            >
              {CATEGORIES.find(c => c.value === existingEntry.category)?.label}
            </span>
          </div>
        )}

        <CategorySelector
          selectedCategory={selectedCategory}
          onSelect={setSelectedCategory}
        />

        <input
          type="text"
          placeholder="Notes (required)"
          value={notes}
          onChange={(e) => setNotes(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && selectedCategory && notes.trim()) {
              handleSave();
            }
            if (e.key === "Escape") {
              onClose();
            }
          }}
          autoFocus
        />

        <div className="time-slot-editor-actions">
          {existingEntry && existingEntry.id && (
            <button className="btn-danger" onClick={handleDelete}>
              Delete
            </button>
          )}
          <div className="action-spacer" />
          <button className="btn-secondary" onClick={onClose}>
            Cancel
          </button>
          <button
            className="btn-primary"
            onClick={handleSave}
            disabled={!selectedCategory || !notes.trim()}
          >
            Save
          </button>
        </div>
      </div>
    </div>
  );
}
