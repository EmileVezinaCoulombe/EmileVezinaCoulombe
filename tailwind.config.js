const { addDynamicIconSelectors } = require("@iconify/tailwind");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      keyframes: {
        wiggle: {
          "0%, 100%": { transform: "rotate(-6deg)" },
          "50%": { transform: "rotate(6deg)" },
        },
        shake: {
          "0%, 100%": { transform: "translate(3px, -3px) rotate(6deg)" },
          "50%": { transform: "translate(-3px, -3px) rotate(-6deg)" },
        },
      },
    },
    animation: {
      wave_hand: "wiggle 0.7s cubic-bezier(.46,.03,.52,.96) infinite",
      shake_fast: "shake 0.3s cubic-bezier(.76,.05,.86,.06) 4",
    },
  },
  plugins: [addDynamicIconSelectors()],
};
