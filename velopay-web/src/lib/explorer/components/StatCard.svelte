<script>
	export let title;
	export let value;
	export let subtitle = '';
	export let icon;
	export let trend = null;
	export let loading = false;
</script>

<div
	class="bg-white dark:bg-neutral-900 rounded-xl p-6 border border-neutral-200 dark:border-neutral-800
            hover:shadow-elevated transition-all duration-200"
>
	<div class="flex items-start justify-between mb-4">
		<div class="flex items-center gap-3">
			{#if icon}
				<div class="p-2 rounded-lg bg-accent-50 dark:bg-accent-950">
					<svelte:component this={icon} class="w-5 h-5 text-accent-600 dark:text-accent-400" />
				</div>
			{/if}
			<span class="text-sm font-medium text-neutral-600 dark:text-neutral-400">{title}</span>
		</div>
		{#if trend}
			<span
				class="text-xs font-semibold px-2 py-1 rounded-full
                   {trend > 0
					? 'bg-success-50 text-success-700 dark:bg-success-950 dark:text-success-400'
					: 'bg-error-50 text-error-700 dark:bg-error-950 dark:text-error-400'}"
			>
				{trend > 0 ? '+' : ''}{trend}%
			</span>
		{/if}
	</div>

	{#if loading}
		<div class="h-8 bg-neutral-200 dark:bg-neutral-800 rounded animate-pulse"></div>
	{:else}
		<div class="text-3xl font-bold text-neutral-900 dark:text-white mb-1">
			{value}
		</div>
	{/if}

	{#if subtitle}
		<div class="text-sm text-neutral-500 dark:text-neutral-400">
			{subtitle}
		</div>
	{/if}
</div>
