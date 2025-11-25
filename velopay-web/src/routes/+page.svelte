<script>
	import { onMount } from 'svelte';
	import { preloadData, goto } from '$app/navigation';

	onMount(async () => {
		const result = await preloadData('/explorer');
		if (result.type === 'loaded' && result.status === 200) {
			goto('/explorer', { replaceState: true });
		} else {
			goto('/explorer', { replaceState: true });
		}
	});
</script>
