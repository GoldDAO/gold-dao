import Preset from "./src/theme/preset";

/** @type {import('tailwindcss').Config} */
export default {
  presets: [Preset],
  darkMode: "media",
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  plugins: [],
};
