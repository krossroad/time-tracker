import { useEffect } from "react";
import { useAppStore } from "../../stores/appStore";

export function SettingsPanel() {
  const { settings, loadSettings, updateSetting, setCurrentView } = useAppStore();

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  const intervalMinutes = settings.interval_minutes || "15";
  const idleThreshold = settings.idle_threshold_minutes || "5";
  const notificationEnabled = settings.notification_enabled !== "false";
  const notificationSound = settings.notification_sound || "default";

  const handleIntervalChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    updateSetting("interval_minutes", e.target.value);
  };

  const handleIdleThresholdChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    updateSetting("idle_threshold_minutes", e.target.value);
  };

  const handleNotificationEnabledChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    updateSetting("notification_enabled", e.target.checked ? "true" : "false");
  };

  const handleNotificationSoundChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    updateSetting("notification_sound", e.target.value);
  };

  return (
    <div className="settings-panel">
      <div className="settings-header">
        <button className="back-btn" onClick={() => setCurrentView("calendar")}>
          &lt; Back
        </button>
        <h2>Settings</h2>
      </div>

      <div className="settings-content">
        <div className="setting-item">
          <label htmlFor="interval">Prompt Interval</label>
          <select
            id="interval"
            value={intervalMinutes}
            onChange={handleIntervalChange}
          >
            <option value="1">1 minute (testing)</option>
            <option value="5">5 minutes</option>
            <option value="10">10 minutes</option>
            <option value="15">15 minutes</option>
            <option value="30">30 minutes</option>
            <option value="60">1 hour</option>
          </select>
          <p className="setting-description">
            How often to prompt for what you're working on
          </p>
        </div>

        <div className="setting-item">
          <label htmlFor="idle">Idle Detection Threshold</label>
          <select
            id="idle"
            value={idleThreshold}
            onChange={handleIdleThresholdChange}
          >
            <option value="2">2 minutes</option>
            <option value="5">5 minutes</option>
            <option value="10">10 minutes</option>
            <option value="15">15 minutes</option>
            <option value="30">30 minutes</option>
          </select>
          <p className="setting-description">
            After this time idle, you'll be marked as "Away"
          </p>
        </div>

        <div className="setting-item">
          <label htmlFor="notification-enabled">
            <input
              id="notification-enabled"
              type="checkbox"
              checked={notificationEnabled}
              onChange={handleNotificationEnabledChange}
            />
            Enable Notifications
          </label>
          <p className="setting-description">
            Show system notifications when prompts appear
          </p>
        </div>

        <div className="setting-item">
          <label htmlFor="notification-sound">Notification Sound</label>
          <select
            id="notification-sound"
            value={notificationSound}
            onChange={handleNotificationSoundChange}
            disabled={!notificationEnabled}
          >
            <option value="default">Default</option>
            <option value="glass">Glass</option>
            <option value="hero">Hero</option>
            <option value="morse">Morse</option>
            <option value="ping">Ping</option>
            <option value="pop">Pop</option>
            <option value="purr">Purr</option>
            <option value="sosumi">Sosumi</option>
            <option value="submarine">Submarine</option>
            <option value="tink">Tink</option>
          </select>
          <p className="setting-description">
            Sound to play with notifications (macOS sounds)
          </p>
        </div>
      </div>

      <div className="settings-footer">
        <p className="app-info">Weekly Tracker v0.1.0</p>
      </div>
    </div>
  );
}
