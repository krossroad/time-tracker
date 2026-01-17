import { useState } from "react";
import { format } from "date-fns";
import { useAppStore } from "../../stores/appStore";
import { CategorySelector } from "./CategorySelector";
import { Category, MissedPrompt } from "../../types";

interface MissedPromptItemProps {
  prompt: MissedPrompt;
  onFill: (timestamp: number, category: Category, notes: string) => void;
}

function MissedPromptItem({ prompt, onFill }: MissedPromptItemProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(null);
  const [notes, setNotes] = useState("");

  const handleSubmit = () => {
    if (!selectedCategory || !notes.trim()) return;
    onFill(prompt.timestamp, selectedCategory, notes);
    setIsExpanded(false);
    setSelectedCategory(null);
    setNotes("");
  };

  return (
    <div className="missed-prompt-item">
      <div
        className="missed-prompt-header"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <span className="missed-time">
          {format(new Date(prompt.timestamp * 1000), "h:mm a")}
        </span>
        <span className="missed-label">Missed</span>
      </div>

      {isExpanded && (
        <div className="missed-prompt-form">
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
                handleSubmit();
              }
            }}
            required
          />
          <button
            className="btn-primary"
            onClick={handleSubmit}
            disabled={!selectedCategory || !notes.trim()}
          >
            Fill In
          </button>
        </div>
      )}
    </div>
  );
}

export function MissedPromptsPanel() {
  const { missedPrompts, fillMissedPrompt } = useAppStore();

  if (missedPrompts.length === 0) {
    return null;
  }

  return (
    <div className="missed-prompts-panel">
      <h3>Missed Prompts ({missedPrompts.length})</h3>
      <div className="missed-prompts-list">
        {missedPrompts.map((prompt) => (
          <MissedPromptItem
            key={prompt.timestamp}
            prompt={prompt}
            onFill={fillMissedPrompt}
          />
        ))}
      </div>
    </div>
  );
}
