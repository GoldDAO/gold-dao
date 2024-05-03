/* eslint-disable global-require */
/** @type {import('tailwindcss').Config} */
const { nextui } = require('@nextui-org/react');

module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
    './node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic':
          'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
      },
      colors: {
        DarkGrey: '#C6C6C6',
        SoftGrey: '#F3F3F3',
        Gold: '#D3B871',
        CardBackground: '#F4F4F4',
      },
      width: {
        15: '62px',
        21: '83px',
        22: '87px',
        86: '350px',
        100: '400px',
        110: '430px',
        120: '472px',
        150: '590px',
      },
      height: {
        17: '68px',
        19: '76px',
        21: '83px',
        22: '88px',
        86: '350px',
        100: '400px',
        120: '472px',
        150: '590px',
      },
      size: {
        25: '100px',
      },
      borderRadius: {
        '4xl': '36px',
      },
      fontSize: {
        '2xs': '10.64px',
        '3xs': '8px',
      },
      padding: {
        17: '67px',
        18: '70px',
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('daisyui'),
    nextui()],
  // darkMode: 'class',
};
