/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		"./index.html",
		"./src/**/*.{rs,html}", // This tells Tailwind to scan all your Rust files!
	],
	theme: {
		extend: {},
	},
	plugins: [],
}
