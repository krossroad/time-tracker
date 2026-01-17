import { CATEGORIES, Category } from "../../types";

interface CategorySelectorProps {
  selectedCategory: Category | null;
  onSelect: (category: Category) => void;
}

export function CategorySelector({
  selectedCategory,
  onSelect,
}: CategorySelectorProps) {
  return (
    <div className="category-grid">
      {CATEGORIES.filter((c) => c.value !== "away").map((category) => (
        <button
          key={category.value}
          className={`category-button ${
            selectedCategory === category.value ? "selected" : ""
          }`}
          style={{
            borderColor: category.color,
            backgroundColor:
              selectedCategory === category.value
                ? category.color
                : "transparent",
            color: selectedCategory === category.value ? "white" : category.color,
          }}
          onClick={() => onSelect(category.value)}
        >
          {category.label}
        </button>
      ))}
    </div>
  );
}
