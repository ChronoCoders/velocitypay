/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	darkMode: 'class',
	theme: {
		extend: {
			colors: {
				primary: {
					50: '#f0f4f8',
					100: '#d9e3ee',
					200: '#b3c7dc',
					300: '#8dabc9',
					400: '#678fb7',
					500: '#4173a4',
					600: '#335d8a',
					700: '#254767',
					800: '#183145',
					900: '#0A2647',
					950: '#051622'
				},
				accent: {
					50: '#e6f0ff',
					100: '#cce0ff',
					200: '#99c1ff',
					300: '#66a2ff',
					400: '#3383ff',
					500: '#0064ff',
					600: '#0050cc',
					700: '#003c99',
					800: '#002866',
					900: '#144272',
					950: '#0a1f3d'
				},
				success: {
					50: '#ecfdf5',
					100: '#d1fae5',
					200: '#a7f3d0',
					300: '#6ee7b7',
					400: '#34d399',
					500: '#10b981',
					600: '#059669',
					700: '#2C7865',
					800: '#065f46',
					900: '#064e3b'
				},
				warning: {
					50: '#fffbeb',
					100: '#fef3c7',
					200: '#fde68a',
					300: '#fcd34d',
					400: '#fbbf24',
					500: '#F4A442',
					600: '#d97706',
					700: '#b45309',
					800: '#92400e',
					900: '#78350f'
				}
			},
			fontFamily: {
				sans: ['Inter', 'system-ui', 'sans-serif'],
				mono: ['JetBrains Mono', 'Consolas', 'monospace']
			},
			boxShadow: {
				glass: '0 8px 32px 0 rgba(31, 38, 135, 0.15)',
				elevated: '0 20px 25px -5px rgba(0, 0, 0, 0.1)'
			}
		}
	},
	plugins: []
};
