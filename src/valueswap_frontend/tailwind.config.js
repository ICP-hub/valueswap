/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      screens: {
        "custom-450": "450px", // Define your custom screen size with its breakpoint
        "custom-400": "400px",
      },
      backgroundImage: {
        "custom-radial":
          "radial-gradient(circle, rgba(153,153,153,1) 0%, rgba(0,48,142,1) 50%)",
          "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
      },
      // backgroundHero: {
      //   "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
      // },
      keyframes: {
        "border-spin": {
          "100%": {
            transform: "rotate(360deg)",
          },
        },
      },
      animation: {
        "border-spin": "border-spin 7s linear infinite",
      },
      
    },
    fontFamily: {
      gilroy: ["gilroy", "sans-serif"],
      cabin: ["cabin", "sans-serif"],
    },
  },
  plugins: [],
};
