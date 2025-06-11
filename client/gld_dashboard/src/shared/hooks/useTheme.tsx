import { useEffect, useState, useCallback } from "react";

export type Theme = "light" | "dark" | "system";

const THEME_KEY = "theme";

export const useTheme = () => {
  const savedTheme = localStorage.getItem(THEME_KEY) as Theme | null;
  const [theme, setTheme] = useState<Theme>(savedTheme || "system");

  const applyTheme = useCallback((theme: Theme, prefersDark: boolean) => {
    const root = window.document.documentElement;
    root.classList.remove("dark", "light");
    if (theme === "dark" || (theme === "system" && prefersDark)) {
      root.classList.add("dark");
    } else if (theme === "light" || (theme === "system" && !prefersDark)) {
      root.classList.add("light");
    }
  }, []);

  useEffect(() => {
    const appliedTheme = savedTheme ?? "system";
    setTheme(appliedTheme);

    const mql = window.matchMedia("(prefers-color-scheme: dark)");
    const handleChange = () => {
      applyTheme(appliedTheme, mql.matches);
    };
    applyTheme(appliedTheme, mql.matches);

    if (appliedTheme === "system") {
      mql.addEventListener("change", handleChange);
      return () => mql.removeEventListener("change", handleChange);
    }
  }, [savedTheme, theme, applyTheme]);

  const changeTheme = (newTheme: Theme) => {
    setTheme(newTheme);
    localStorage.setItem(THEME_KEY, newTheme);
    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)"
    ).matches;
    applyTheme(newTheme, prefersDark);
  };

  return { theme, changeTheme };
};
