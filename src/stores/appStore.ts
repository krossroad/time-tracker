import { create } from "zustand";
import {
  TimeEntry,
  MissedPrompt,
  ViewMode,
  Category,
} from "../types";
import * as api from "../services/api";
import { startOfDay, endOfDay } from "date-fns";

interface AppState {
  currentView: "prompt" | "calendar" | "settings";
  selectedDate: Date;
  viewMode: ViewMode;
  pendingTimestamp: number | null;
  entries: TimeEntry[];
  missedPrompts: MissedPrompt[];
  settings: Record<string, string>;
  isLoading: boolean;

  // Actions
  setCurrentView: (view: "prompt" | "calendar" | "settings") => void;
  setSelectedDate: (date: Date) => void;
  setViewMode: (mode: ViewMode) => void;
  setPendingTimestamp: (timestamp: number | null) => void;

  // Data actions
  loadEntriesForDate: (date: Date) => Promise<void>;
  loadMissedPrompts: (date: Date) => Promise<void>;
  loadSettings: () => Promise<void>;

  createEntry: (timestamp: number, category: Category, notes: string, isRetroactive?: boolean) => Promise<void>;
  deleteEntry: (id: number) => Promise<void>;
  updateEntry: (id: number, category?: Category, notes?: string) => Promise<void>;

  updateSetting: (key: string, value: string) => Promise<void>;

  fillMissedPrompt: (timestamp: number, category: Category, notes: string) => Promise<void>;
}

export const useAppStore = create<AppState>((set, get) => ({
  currentView: "calendar",
  selectedDate: new Date(),
  viewMode: "timeline",
  pendingTimestamp: null,
  entries: [],
  missedPrompts: [],
  settings: {},
  isLoading: false,

  setCurrentView: (view) => set({ currentView: view }),
  setSelectedDate: (date) => set({ selectedDate: date }),
  setViewMode: (mode) => set({ viewMode: mode }),
  setPendingTimestamp: (timestamp) => {
    set({ pendingTimestamp: timestamp });
    if (timestamp) {
      set({ currentView: "prompt" });
    }
  },

  loadEntriesForDate: async (date) => {
    set({ isLoading: true });
    try {
      const start = Math.floor(startOfDay(date).getTime() / 1000);
      const end = Math.floor(endOfDay(date).getTime() / 1000);
      const entries = await api.getEntriesForDate(start, end);
      set({ entries, isLoading: false });
    } catch (error) {
      console.error("Failed to load entries:", error);
      set({ isLoading: false });
    }
  },

  loadMissedPrompts: async (date) => {
    try {
      const start = Math.floor(startOfDay(date).getTime() / 1000);
      const end = Math.floor(endOfDay(date).getTime() / 1000);
      const missedPrompts = await api.getMissedPrompts(start, end);
      set({ missedPrompts });
    } catch (error) {
      console.error("Failed to load missed prompts:", error);
    }
  },

  loadSettings: async () => {
    try {
      const settingsArray = await api.getAllSettings();
      const settings: Record<string, string> = {};
      settingsArray.forEach((s) => {
        settings[s.key] = s.value;
      });
      set({ settings });
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  },

  createEntry: async (timestamp, category, notes, isRetroactive = false) => {
    try {
      await api.createTimeEntry(timestamp, category, {
        notes,
        is_retroactive: isRetroactive,
      });
      await get().loadEntriesForDate(get().selectedDate);
      set({ pendingTimestamp: null, currentView: "calendar" });
    } catch (error) {
      console.error("Failed to create entry:", error);
    }
  },

  deleteEntry: async (id) => {
    try {
      await api.deleteTimeEntry(id);
      await get().loadEntriesForDate(get().selectedDate);
    } catch (error) {
      console.error("Failed to delete entry:", error);
    }
  },

  updateEntry: async (id, category, notes) => {
    try {
      await api.updateTimeEntry(id, { category, notes });
      await get().loadEntriesForDate(get().selectedDate);
    } catch (error) {
      console.error("Failed to update entry:", error);
    }
  },

  updateSetting: async (key, value) => {
    try {
      await api.setSetting(key, value);
      set((state) => ({
        settings: { ...state.settings, [key]: value },
      }));
    } catch (error) {
      console.error("Failed to update setting:", error);
    }
  },

  fillMissedPrompt: async (timestamp, category, notes) => {
    try {
      await api.createTimeEntry(timestamp, category, {
        notes,
        is_retroactive: true,
      });
      await api.deleteMissedPrompt(timestamp);
      await get().loadEntriesForDate(get().selectedDate);
      await get().loadMissedPrompts(get().selectedDate);
    } catch (error) {
      console.error("Failed to fill missed prompt:", error);
    }
  },
}));
