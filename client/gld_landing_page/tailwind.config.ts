import type { Config } from 'tailwindcss'

// Colors and typefaces are the ones of the Gold DAO dApp
// (client/gld_dashboard/src/App.css), so both sites read as one product.
const config: Config = {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        background: 'oklch(99.16% 0.0017 67.8 / <alpha-value>)',
        content: 'oklch(0% 0 0 / <alpha-value>)',
        line: 'oklch(92.51% 0.0071 88.65 / <alpha-value>)',
        gold: 'oklch(79.05% 0.0944 89.09 / <alpha-value>)'
      },
      fontFamily: {
        sans: ['polymath', 'system-ui', 'sans-serif'],
        display: ['polymath-display', 'polymath', 'system-ui', 'sans-serif']
      }
    }
  },
  plugins: []
}

export default config
