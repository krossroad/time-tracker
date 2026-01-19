import { useMemo, useState } from "react";
import { format, startOfDay, addMinutes } from "date-fns";
import { useAppStore } from "../../stores/appStore";
import { CATEGORIES, TimeEntry } from "../../types";
import { TimeSlotEditor } from "./TimeSlotEditor";

interface TimeSlot {
  timestamp: number;
  entry: TimeEntry | null;
}

export function DayTimeline() {
  const { selectedDate, entries } = useAppStore();
  const [editingSlot, setEditingSlot] = useState<TimeSlot | null>(null);

  const timeSlots = useMemo(() => {
    const slots: TimeSlot[] = [];
    const dayStart = startOfDay(selectedDate);

    // Create 96 slots (24 hours * 4 slots per hour)
    for (let i = 0; i < 96; i++) {
      const slotTime = addMinutes(dayStart, i * 15);
      const timestamp = Math.floor(slotTime.getTime() / 1000);

      const entry = entries.find((e) => e.timestamp === timestamp);

      slots.push({
        timestamp,
        entry: entry || null,
      });
    }

    return slots;
  }, [selectedDate, entries]);

  const getCategoryColor = (category: string) => {
    const cat = CATEGORIES.find((c) => c.value === category);
    return cat?.color || "#E5E7EB";
  };

  // Group slots by hour for display
  const hourGroups = useMemo(() => {
    const groups: { hour: number; slots: TimeSlot[] }[] = [];
    for (let hour = 0; hour < 24; hour++) {
      groups.push({
        hour,
        slots: timeSlots.slice(hour * 4, (hour + 1) * 4),
      });
    }
    return groups;
  }, [timeSlots]);

  const handleSlotClick = (slot: TimeSlot) => {
    setEditingSlot(slot);
  };

  return (
    <div className="day-timeline">
      {hourGroups.map((group) => (
        <div key={group.hour} className="hour-row">
          <div className="hour-label">
            {format(addMinutes(startOfDay(selectedDate), group.hour * 60), "ha")}
          </div>
          <div className="quarter-slots">
            {group.slots.map((slot) => (
              <div
                key={slot.timestamp}
                className={`quarter-slot ${slot.entry ? "filled" : ""} clickable`}
                style={{
                  backgroundColor: slot.entry
                    ? getCategoryColor(slot.entry.category)
                    : undefined,
                }}
                title={
                  slot.entry
                    ? `${CATEGORIES.find((c) => c.value === slot.entry!.category)?.label || slot.entry.category}: ${slot.entry.notes || ""}`
                    : format(new Date(slot.timestamp * 1000), "h:mm a")
                }
                onClick={() => handleSlotClick(slot)}
              />
            ))}
          </div>
        </div>
      ))}

      {editingSlot && (
        <TimeSlotEditor
          timestamp={editingSlot.timestamp}
          existingEntry={editingSlot.entry}
          onClose={() => setEditingSlot(null)}
        />
      )}
    </div>
  );
}
