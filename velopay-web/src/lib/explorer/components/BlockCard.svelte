<script>
	import Box from 'lucide-svelte/icons/box';
	import Clock from 'lucide-svelte/icons/clock';
	import Hash from 'lucide-svelte/icons/hash';
	import { preloadData, pushState, goto } from '$app/navigation';

	export let block;

	function formatTime(timestamp) {
		const now = Date.now();
		const diff = Math.floor((now - timestamp) / 1000);
		if (diff < 60) return `${diff}s ago`;
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		return `${Math.floor(diff / 3600)}h ago`;
	}

	function truncateHash(hash) {
		return `${hash.slice(0, 10)}...${hash.slice(-8)}`;
	}

	async function handleClick(event) {
		event.preventDefault();
		const href = `/explorer/block/${block.number}`;
		const result = await preloadData(href);
		if (result.type === 'loaded' && result.status === 200) {
			pushState(href, { selected: block });
		} else {
			goto(href);
		}
	}
</script>

<a
	href="/explorer/block/{block.number}"
	on:click={handleClick}
	class="block w-full bg-white dark:bg-neutral-900 rounded-lg p-4 border border-neutral-200 dark:border-neutral-800 hover:border-accent-500 dark:hover:border-accent-500 hover:shadow-lg transition-all duration-200 group no-underline"
>
	<div class="flex items-center justify-between mb-3">
		<div class="flex items-center gap-3">
			<div
				class="p-2 rounded-lg bg-primary-50 dark:bg-primary-950 group-hover:bg-accent-50 dark:group-hover:bg-accent-950 transition-colors"
			>
				<Box
					class="w-4 h-4 text-primary-600 dark:text-primary-400 group-hover:text-accent-600 dark:group-hover:text-accent-400 transition-colors"
				/>
			</div>
			<div>
				<div class="text-sm font-mono font-semibold text-neutral-900 dark:text-white">
					#{block.number}
				</div>
				<div class="text-xs text-neutral-500 dark:text-neutral-400 flex items-center gap-1">
					<Clock class="w-3 h-3" />
					{formatTime(block.timestamp || Date.now())}
				</div>
			</div>
		</div>

		<div class="text-right">
			<div class="text-xs text-neutral-600 dark:text-neutral-400 mb-1">
				{block.extrinsics?.length || 0} extrinsics
			</div>
			<div
				class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium {block.status ===
				'finalized'
					? 'bg-success-50 text-success-700 dark:bg-success-950 dark:text-success-400'
					: 'bg-warning-50 text-warning-700 dark:bg-warning-950 dark:text-warning-400'}"
			>
				<div
					class="w-1.5 h-1.5 rounded-full {block.status === 'finalized'
						? 'bg-success-500'
						: 'bg-warning-500'}"
				></div>
				{block.status || 'pending'}
			</div>
		</div>
	</div>

	<div class="flex items-center gap-2 text-xs font-mono text-neutral-500 dark:text-neutral-400">
		<Hash class="w-3 h-3" />
		<span class="group-hover:text-accent-600 dark:group-hover:text-accent-400 transition-colors">
			{truncateHash(block.hash)}
		</span>
	</div>
</a>
