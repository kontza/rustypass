/** @type {import('tailwindcss').Config}*/
const config = {
  content: ['./src/**/*.{html,js,svelte}'],

  theme: {
    extend: {},
  },

  daisyui: {
    themes: ['light', 'dark'],
  },

  plugins: [require('daisyui')],
}

module.exports = config
