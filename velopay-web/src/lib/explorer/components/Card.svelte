<script>
	export let glass = false;
	export let elevated = false;
</script>

<div
	class="rounded-xl border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900
            {glass ? 'backdrop-blur-lg bg-white/80 dark:bg-neutral-900/80 shadow-glass' : ''} 
            {elevated ? 'shadow-elevated' : 'shadow-sm'} 
            transition-all duration-200"
>
	<slot />
</div>
