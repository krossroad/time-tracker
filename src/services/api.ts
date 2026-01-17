import { invoke } from "@tauri-apps/api/core";
import { TimeEntry, MissedPrompt, Setting, Category } from "../types";

export async function createTimeEntry(
  timestamp: number,
  category: Category,
  options?: {
    duration_minutes?: number;
    is_away?: boolean;
    is_retroactive?: boolean;
    notes?: string;
  }
): Promise<number> {
  return invoke<number>("create_time_entry", {
    timestamp,
    category,
    durationMinutes: options?.duration_minutes,
    isAway: options?.is_away,
    isRetroactive: options?.is_retroactive,
    notes: options?.notes,
  });
}

export async function getEntriesForDate(
  startTimestamp: number,
  endTimestamp: number
): Promise<TimeEntry[]> {
  return invoke<TimeEntry[]>("get_entries_for_date", {
    startTimestamp,
    endTimestamp,
  });
}

export async function updateTimeEntry(
  id: number,
  options: { category?: Category; notes?: string }
): Promise<void> {
  return invoke("update_time_entry", {
    id,
    category: options.category,
    notes: options.notes,
  });
}

export async function deleteTimeEntry(id: number): Promise<void> {
  return invoke("delete_time_entry", { id });
}

export async function createMissedPrompt(
  timestamp: number,
  reason?: string
): Promise<number> {
  return invoke<number>("create_missed_prompt", { timestamp, reason });
}

export async function getMissedPrompts(
  startTimestamp: number,
  endTimestamp: number
): Promise<MissedPrompt[]> {
  return invoke<MissedPrompt[]>("get_missed_prompts", {
    startTimestamp,
    endTimestamp,
  });
}

export async function deleteMissedPrompt(timestamp: number): Promise<void> {
  return invoke("delete_missed_prompt", { timestamp });
}

export async function getSetting(key: string): Promise<string | null> {
  return invoke<string | null>("get_setting", { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  return invoke("set_setting", { key, value });
}

export async function getAllSettings(): Promise<Setting[]> {
  return invoke<Setting[]>("get_all_settings");
}
