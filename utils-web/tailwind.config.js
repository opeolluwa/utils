/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },

  colors: {
    "amber": {
      DEFAULT: "#FCB900",
      50: "#FFEBB5",
      100: "#FFE6A0",
      200: "#FFDB77",
      300: "#FFD04F",
      400: "#FFC526",
      500: "#FCB900",
      600: "#C49000",
      700: "#8C6700",
      800: "#543D00",
      900: "#1C1400",
      950: "#000000",
    },
    ...colors,
  },
  theme: {
    container: {
      center: true,
      padding: {
        DEFAULT: "1rem",
        sm: "2rem",
        lg: "4rem",
        xl: "5rem",
        "2xl": "6rem",
      },
      screens: {
        sm: "100%",
        md: "100%",
        lg: "1024px",
        xl: "1280px",
      },
      section: {
        margin: "100px 0",
        screens: {
          sm: "100%",
          md: "100%",
          lg: "1024px",
          xl: "1280px",
        },
      },
    },
    extend: {},
  },
  plugins: [
    require("@tailwindcss/forms"),
    // ...
  ],
};
