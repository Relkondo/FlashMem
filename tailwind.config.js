/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        dark: '#121212',
        customGreen: '#10B981',
      }
    },
  },
  plugins: [],
}