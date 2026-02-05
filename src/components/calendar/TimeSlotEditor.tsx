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
  const [isEditing, setIsEditing] = useState(false);

  const intervalMinutes = parseInt(settings.interval_minutes || "15", 10);
  const intervalStart = format(new Date(timestamp * 1000), "h:mm a");
  const intervalEnd = format(new Date((timestamp + intervalMinutes * 60) * 1000), "h:mm a");

  useEffect(() => {
    if (existingEntry) {
      setSelectedCategory(existingEntry.category);
      setNotes(existingEntry.notes || "");
    }
  }, [existingEntry]);

  const handleEditClick = () => {
    setIsEditing(true);
  };

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

  const categoryInfo = existingEntry
    ? CATEGORIES.find(c => c.value === existingEntry.category)
    : null;

  return (
    <div className="time-slot-editor-overlay" onClick={handleOverlayClick}>
      <div className="time-slot-editor">
        <div className="time-slot-editor-header">
          <h3>{isEditing ? (existingEntry ? "Edit Time Slot" : "Fill Time Slot") : "Time Slot"}</h3>
          <div className="header-actions">
            {!isEditing && (
              <button className="edit-btn" onClick={handleEditClick} title="Edit entry">
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M11.013 1.427a1.75 1.75 0 0 1 2.474 0l1.086 1.086a1.75 1.75 0 0 1 0 2.474l-8.61 8.61c-.21.21-.47.364-.756.442l-3.251.93a.75.75 0 0 1-.927-.928l.929-3.25a1.75 1.75 0 0 1 .443-.757l8.612-8.607Zm1.414 1.06a.25.25 0 0 0-.354 0L3.462 11.1a.25.25 0 0 0-.063.108l-.558 1.953 1.953-.558a.25.25 0 0 0 .108-.063l8.61-8.607a.25.25 0 0 0 0-.354l-1.086-1.086Z" fill="currentColor"/>
                </svg>
              </button>
            )}
            <button className="close-btn" onClick={onClose}>×</button>
          </div>
        </div>

        <p className="time-slot-interval">{intervalStart} - {intervalEnd}</p>

        {isEditing ? (
          <>
            {existingEntry && (
              <div className="current-entry-info">
                <span
                  className="current-category-badge"
                  style={{ backgroundColor: categoryInfo?.color }}
                >
                  {categoryInfo?.label}
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
          </>
        ) : (
          <div className="time-slot-view">
            {existingEntry ? (
              <>
                <div className="view-field">
                  <span className="view-label">Category</span>
                  <span
                    className="current-category-badge"
                    style={{ backgroundColor: categoryInfo?.color }}
                  >
                    {categoryInfo?.label}
                  </span>
                </div>
                {existingEntry.notes && (
                  <div className="view-field">
                    <span className="view-label">Notes</span>
                    <span className="view-value">{existingEntry.notes}</span>
                  </div>
                )}
              </>
            ) : (
              <p className="view-empty">No entry for this time slot.</p>
            )}
            <div className="time-slot-editor-actions">
              <div className="action-spacer" />
              <button className="btn-secondary" onClick={onClose}>
                Close
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
