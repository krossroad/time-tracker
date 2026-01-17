import { useEffect } from "react";
import { format, addDays, subDays, isToday } from "date-fns";
import { useAppStore } from "../../stores/appStore";
import { DaySummary } from "./DaySummary";
import { DayTimeline } from "./DayTimeline";
import { MissedPromptsPanel } from "../tracker/MissedPromptsPanel";
import { ExportButton } from "./ExportButton";

export function CalendarView() {
  const {
    selectedDate,
    setSelectedDate,
    viewMode,
    setViewMode,
    loadEntriesForDate,
    loadMissedPrompts,
  } = useAppStore();

  useEffect(() => {
    loadEntriesForDate(selectedDate);
    loadMissedPrompts(selectedDate);
  }, [selectedDate, loadEntriesForDate, loadMissedPrompts]);

  const goToPreviousDay = () => {
    setSelectedDate(subDays(selectedDate, 1));
  };

  const goToNextDay = () => {
    setSelectedDate(addDays(selectedDate, 1));
  };

  const goToToday = () => {
    setSelectedDate(new Date());
  };

  return (
    <div className="calendar-view">
      <div className="calendar-header">
        <div className="date-nav">
          <button className="nav-btn" onClick={goToPreviousDay}>
            &lt;
          </button>
          <div className="current-date">
            <span className="date-display">
              {format(selectedDate, "EEEE, MMMM d, yyyy")}
            </span>
            {!isToday(selectedDate) && (
              <button className="today-btn" onClick={goToToday}>
                Today
              </button>
            )}
          </div>
          <button className="nav-btn" onClick={goToNextDay}>
            &gt;
          </button>
        </div>

        <div className="header-actions">
          <div className="view-toggle">
            <button
              className={`toggle-btn ${viewMode === "summary" ? "active" : ""}`}
              onClick={() => setViewMode("summary")}
            >
              Summary
            </button>
            <button
              className={`toggle-btn ${viewMode === "timeline" ? "active" : ""}`}
              onClick={() => setViewMode("timeline")}
            >
              Timeline
            </button>
          </div>
          <ExportButton />
        </div>
      </div>

      <MissedPromptsPanel />

      <div className="calendar-content">
        {viewMode === "summary" ? <DaySummary /> : <DayTimeline />}
      </div>
    </div>
  );
}
