/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{svelte,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // Cyberpunk theme colors
        cyber: {
          bg: "#0a0a0f",
          panel: "#111827",
          card: "#1f2937",
          border: "#374151",
        },
        neon: {
          green: "#00ff9d",
          blue: "#00d2ff",
          pink: "#ff0055",
          yellow: "#ffee00",
        },
      },
      fontFamily: {
        mono: [
          "JetBrains Mono",
          "Fira Code",
          "SF Mono",
          "Consolas",
          "monospace",
        ],
      },
      boxShadow: {
        neon: "0 0 5px theme('colors.neon.green'), 0 0 20px theme('colors.neon.green')",
        "neon-blue":
          "0 0 5px theme('colors.neon.blue'), 0 0 20px theme('colors.neon.blue')",
        "neon-pink":
          "0 0 5px theme('colors.neon.pink'), 0 0 20px theme('colors.neon.pink')",
      },
      animation: {
        glow: "glow 2s ease-in-out infinite alternate",
        "pulse-slow": "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite",
      },
      keyframes: {
        glow: {
          "0%": { boxShadow: "0 0 5px #00ff9d, 0 0 10px #00ff9d" },
          "100%": { boxShadow: "0 0 10px #00ff9d, 0 0 30px #00ff9d" },
        },
      },
    },
  },
  plugins: [],
};
