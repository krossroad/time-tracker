import { useMemo } from "react";
import { PieChart, Pie, Cell, Legend, ResponsiveContainer, Tooltip } from "recharts";
import { useAppStore } from "../../stores/appStore";
import { CATEGORIES } from "../../types";

interface CategorySummary {
  name: string;
  value: number;
  color: string;
  [key: string]: string | number;
}

export function DaySummary() {
  const { entries } = useAppStore();

  const summaryData = useMemo(() => {
    const categoryMinutes: Record<string, number> = {};

    entries.forEach((entry) => {
      const minutes = entry.duration_minutes || 15;
      categoryMinutes[entry.category] =
        (categoryMinutes[entry.category] || 0) + minutes;
    });

    const data: CategorySummary[] = [];

    CATEGORIES.forEach((category) => {
      const minutes = categoryMinutes[category.value] || 0;
      if (minutes > 0) {
        data.push({
          name: category.label,
          value: minutes,
          color: category.color,
        });
      }
    });

    return data;
  }, [entries]);

  const totalMinutes = summaryData.reduce((sum, item) => sum + item.value, 0);
  const totalHours = Math.floor(totalMinutes / 60);
  const remainingMinutes = totalMinutes % 60;

  if (summaryData.length === 0) {
    return (
      <div className="day-summary empty">
        <p>No entries for this day</p>
      </div>
    );
  }

  const formatTime = (minutes: number) => {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    if (hours > 0 && mins > 0) {
      return `${hours}h ${mins}m`;
    } else if (hours > 0) {
      return `${hours}h`;
    } else {
      return `${mins}m`;
    }
  };

  return (
    <div className="day-summary">
      <div className="summary-total">
        Total tracked: {totalHours > 0 ? `${totalHours}h ` : ""}
        {remainingMinutes > 0 ? `${remainingMinutes}m` : ""}
      </div>

      <ResponsiveContainer width="100%" height={250}>
        <PieChart>
          <Pie
            data={summaryData}
            cx="50%"
            cy="50%"
            innerRadius={40}
            outerRadius={80}
            paddingAngle={2}
            dataKey="value"
          >
            {summaryData.map((entry, index) => (
              <Cell key={`cell-${index}`} fill={entry.color} />
            ))}
          </Pie>
          <Tooltip
            formatter={(value) => formatTime(value as number)}
          />
          <Legend
            formatter={(value) => {
              const item = summaryData.find((d) => d.name === value);
              return `${value} (${formatTime(item?.value || 0)})`;
            }}
          />
        </PieChart>
      </ResponsiveContainer>

      <div className="category-breakdown">
        {summaryData.map((item) => (
          <div key={item.name} className="breakdown-item">
            <span
              className="breakdown-color"
              style={{ backgroundColor: item.color }}
            />
            <span className="breakdown-name">{item.name}</span>
            <span className="breakdown-time">{formatTime(item.value)}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
