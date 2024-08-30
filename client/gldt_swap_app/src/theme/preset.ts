export const colors = {
  background: "rgb(var(--color-background))",
  content: "rgb(var(--color-content))",
  surface: {
    1: "rgb(var(--color-surface-1))",
    2: "rgb(var(--color-surface-2))",
    DEFAULT: "rgb(var(--color-surface-1))",
  },
  border: "rgb(var(--color-border))",
  gold: "rgb(var(--color-gold))",
  accent: "rgb(var(--color-accent))",
  charcoal: "rgb(var(--color-charcoal))",
  jade: "rgb(var(--color-jade))",
  "dark-orange": "rgb(var(--color-dark-orange))"
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const tailwindcssColors = (colors:any) => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const updatedColors: any = {};
  Object.keys(colors).forEach(key => {
    if (typeof colors[key] === 'object') {
      updatedColors[key] = tailwindcssColors(colors[key]);
    } else if (typeof colors[key] === 'string') {
      updatedColors[key] = colors[key].replace(/\)$/, " / <alpha-value>)");
    }
  });
  return updatedColors;
};

/** @type {import('tailwindcss').Config} */
export default {
  theme: {
    extend: {
      colors: tailwindcssColors(colors),
      fontFamily: {
        sans: ["Inter", "Montserrat", "system-ui", "sans-serif"],
      },
      backgroundImage: {
        'cover-img': "url('/src/assets/bg-cover.png')",
      }
    },
  },
  plugins: [
    import("@tailwindcss/typography"),
    import("@tailwindcss/aspect-ratio"),
    import("@tailwindcss/line-clamp")
  ],
};
