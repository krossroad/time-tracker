export interface TimeEntry {
  id: number | null;
  timestamp: number;
  category: Category;
  duration_minutes: number;
  is_away: boolean;
  is_retroactive: boolean;
  notes: string | null;
  created_at: number | null;
}

export interface MissedPrompt {
  id: number | null;
  timestamp: number;
  reason: string | null;
  created_at: number | null;
}

export interface Setting {
  key: string;
  value: string;
}

export type Category =
  | "deep_work"
  | "meetings"
  | "email"
  | "admin"
  | "break"
  | "personal"
  | "away";

export const CATEGORIES: { value: Category; label: string; color: string }[] = [
  { value: "deep_work", label: "Deep Work", color: "#4F46E5" },
  { value: "meetings", label: "Meetings", color: "#7C3AED" },
  { value: "email", label: "Email", color: "#2563EB" },
  { value: "admin", label: "Admin", color: "#0891B2" },
  { value: "break", label: "Break", color: "#059669" },
  { value: "personal", label: "Personal", color: "#D97706" },
  { value: "away", label: "Away", color: "#6B7280" },
];

export type ViewMode = "summary" | "timeline";

export type ExportDateRange = "this_week" | "last_7_days" | "last_30_days" | "all_time";

export const EXPORT_DATE_RANGES: { value: ExportDateRange; label: string }[] = [
  { value: "this_week", label: "This Week" },
  { value: "last_7_days", label: "Last 7 Days" },
  { value: "last_30_days", label: "Last 30 Days" },
  { value: "all_time", label: "All Time" },
];

export interface AppState {
  currentView: "prompt" | "calendar" | "settings";
  selectedDate: Date;
  viewMode: ViewMode;
  pendingTimestamp: number | null;
  entries: TimeEntry[];
  missedPrompts: MissedPrompt[];
  settings: Record<string, string>;
}
