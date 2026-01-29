/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{svelte,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        modern: {
          primary: "#2563eb",
          secondary: "#64748b",
          accent: "#f59e0b",
          success: "#10b981",
          warning: "#f59e0b",
          danger: "#dc2626",
          bg: "#f8fafc",
          surface: "#ffffff",
          border: "#e2e8f0",
        },
        cyber: {
          bg: "var(--color-bg-base)",
          panel: "var(--color-bg-panel)",
          card: "var(--color-bg-card)",
          border: "var(--color-border)",
        },
        neon: {
          green: "var(--color-neon-green)",
          blue: "var(--color-neon-blue)",
          pink: "var(--color-neon-pink)",
          yellow: "var(--color-neon-yellow)",
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
