import { useEffect, useState } from "react";

export type Theme = "light" | "dark" | "system";
const THEME_KEY = "theme";

function getSavedTheme(): Theme {
  const saved = localStorage.getItem(THEME_KEY) as Theme | null;
  if (saved) return saved;
  return "system";
}

function getActiveTheme(theme: Theme): "light" | "dark" {
  if (theme === "system") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  }
  return theme;
}

function setDataTheme(theme: "light" | "dark") {
  document.documentElement.setAttribute("data-theme", theme);
}

export function useTheme() {
  const [theme, setTheme] = useState<Theme>(getSavedTheme());

  useEffect(() => {
    const active = getActiveTheme(theme);
    setDataTheme(active);

    if (theme === "system") {
      const mql = window.matchMedia("(prefers-color-scheme: dark)");
      const handler = () => setDataTheme(getActiveTheme("system"));
      mql.addEventListener("change", handler);
      return () => mql.removeEventListener("change", handler);
    }
  }, [theme]);

  const changeTheme = (newTheme: Theme) => {
    setTheme(newTheme);
    localStorage.setItem(THEME_KEY, newTheme);
    setDataTheme(getActiveTheme(newTheme));
  };

  return { theme, changeTheme };
}
