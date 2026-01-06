import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        hydrix: {
          DEFAULT: "#3B82F6", // Blue
          light: "#60A5FA",
          dark: "#2563EB",
        },
        serhant: {
          DEFAULT: "#F59E0B", // Amber
          light: "#FBBF24",
          dark: "#D97706",
        },
        mio: {
          DEFAULT: "#EC4899", // Pink
          light: "#F472B6",
          dark: "#DB2777",
        },
        karen: {
          DEFAULT: "#8B5CF6", // Purple
          light: "#A78BFA",
          dark: "#7C3AED",
        },
      },
    },
  },
  plugins: [],
};

export default config;
