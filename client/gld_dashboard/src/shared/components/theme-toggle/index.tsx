// src/components/ThemeToggle.tsx
import { useTheme, Theme } from "@shared/hooks/useTheme";
import { Sun1, Moon, Monitor } from "iconsax-react";

const themes: { label: Theme; Icon: React.ElementType }[] = [
  { label: "light", Icon: Sun1 },
  { label: "dark", Icon: Moon },
  { label: "system", Icon: Monitor },
];

export const ThemeToggle = () => {
  const { theme, changeTheme } = useTheme();

  return (
    <div className="flex gap-2 bg-surface-primary p-1 rounded-xl border border-border">
      {themes.map(({ label, Icon }) => (
        <button
          key={label}
          onClick={() => changeTheme(label)}
          className={`p-2 rounded-lg cursor-pointer bg-surface-primary hover:bg-surface-secondary ${
            theme === label ? "bg-surface-secondary" : ""
          }`}
        >
          <Icon
            size={20}
            variant={theme === label ? "Bold" : "Outline"}
            className="text-content"
          />
        </button>
      ))}
    </div>
  );
};
