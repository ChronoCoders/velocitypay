<script>
	import { onMount } from 'svelte';
	import Moon from 'lucide-svelte/icons/moon';
	import Sun from 'lucide-svelte/icons/sun';

	let isDark = $state(false);
	let mounted = $state(false);

	onMount(() => {
		// Check localStorage and system preference
		const savedTheme = localStorage.getItem('darkMode');

		if (savedTheme !== null) {
			isDark = savedTheme === 'true';
		} else {
			// Check system preference
			isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		}

		// Apply initial theme
		applyTheme(isDark);
		mounted = true;
	});

	function applyTheme(dark) {
		if (dark) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}

	function toggleTheme() {
		isDark = !isDark;
		applyTheme(isDark);
		localStorage.setItem('darkMode', isDark.toString());
	}
</script>

{#if mounted}
	<button
		onclick={toggleTheme}
		aria-label={isDark ? 'Switch to light mode' : 'Switch to dark mode'}
		class="fixed bottom-6 right-6 z-50 w-14 h-14 rounded-full
           bg-white dark:bg-neutral-800
           border-2 border-neutral-200 dark:border-neutral-700
           shadow-lg hover:shadow-xl
           flex items-center justify-center
           transition-all duration-300 ease-in-out
           hover:scale-110 active:scale-95
           focus:outline-none focus:ring-2 focus:ring-accent-500 focus:ring-offset-2
           dark:focus:ring-offset-neutral-900"
	>
		{#if isDark}
			<Sun
				class="w-6 h-6 text-amber-500 transition-transform duration-300 rotate-0 hover:rotate-180"
			/>
		{:else}
			<Moon
				class="w-6 h-6 text-primary-600 transition-transform duration-300 rotate-0 hover:-rotate-12"
			/>
		{/if}
	</button>
{/if}

<style>
	button {
		backdrop-filter: blur(8px);
	}
</style>
