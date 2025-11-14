/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        // White-labelable primary colors - use brand CSS variables
        primary: {
          DEFAULT: "var(--brand-primary)",
          hover: "var(--brand-primary-hover)",
          dark: "var(--brand-primary-dark)",
        },
      },
      fontFamily: {
        sans: ["var(--brand-font-family)", "system-ui", "sans-serif"],
      },
    },
  },
};

