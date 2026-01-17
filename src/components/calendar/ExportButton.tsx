import { useState, useRef, useEffect } from "react";
import { startOfWeek, subDays, format } from "date-fns";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import { exportEntriesToCsv } from "../../services/api";
import { ExportDateRange, EXPORT_DATE_RANGES } from "../../types";

export function ExportButton() {
  const [isOpen, setIsOpen] = useState(false);
  const [isExporting, setIsExporting] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    }

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const getDateRange = (range: ExportDateRange): { start: number; end: number } => {
    const now = new Date();
    const endOfToday = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
    const endTimestamp = Math.floor(endOfToday.getTime() / 1000);

    switch (range) {
      case "this_week": {
        const weekStart = startOfWeek(now, { weekStartsOn: 1 }); // Monday
        return {
          start: Math.floor(weekStart.getTime() / 1000),
          end: endTimestamp,
        };
      }
      case "last_7_days": {
        const sevenDaysAgo = subDays(now, 6);
        const startOfDay = new Date(sevenDaysAgo.getFullYear(), sevenDaysAgo.getMonth(), sevenDaysAgo.getDate());
        return {
          start: Math.floor(startOfDay.getTime() / 1000),
          end: endTimestamp,
        };
      }
      case "last_30_days": {
        const thirtyDaysAgo = subDays(now, 29);
        const startOfDay = new Date(thirtyDaysAgo.getFullYear(), thirtyDaysAgo.getMonth(), thirtyDaysAgo.getDate());
        return {
          start: Math.floor(startOfDay.getTime() / 1000),
          end: endTimestamp,
        };
      }
      case "all_time":
        return {
          start: 0,
          end: endTimestamp,
        };
    }
  };

  const handleExport = async (range: ExportDateRange) => {
    setIsOpen(false);
    setIsExporting(true);

    try {
      const { start, end } = getDateRange(range);
      const csvContent = await exportEntriesToCsv(start, end);

      const rangeLabel = EXPORT_DATE_RANGES.find((r) => r.value === range)?.label || range;
      const defaultFileName = `time-tracker-${rangeLabel.toLowerCase().replace(/\s+/g, "-")}-${format(new Date(), "yyyy-MM-dd")}.csv`;

      const filePath = await save({
        defaultPath: defaultFileName,
        filters: [
          {
            name: "CSV",
            extensions: ["csv"],
          },
        ],
      });

      if (filePath) {
        await writeTextFile(filePath, csvContent);
      }
    } catch (error) {
      console.error("Export failed:", error);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="export-dropdown" ref={dropdownRef}>
      <button
        className="export-btn"
        onClick={() => setIsOpen(!isOpen)}
        disabled={isExporting}
      >
        {isExporting ? "Exporting..." : "Export"}
      </button>
      {isOpen && (
        <div className="export-menu">
          {EXPORT_DATE_RANGES.map((range) => (
            <button
              key={range.value}
              className="export-menu-item"
              onClick={() => handleExport(range.value)}
            >
              {range.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
