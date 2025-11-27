<script>
	import { onMount } from 'svelte';
	import {
		connectToChain,
		subscribeToBlocks,
		isConnected,
		chainInfo
	} from '$lib/explorer/api/chainApi';
	import StatCard from '$lib/explorer/components/StatCard.svelte';
	import BlockCard from '$lib/explorer/components/BlockCard.svelte';
	import { Activity, Blocks, Users, TrendingUp, Search } from 'lucide-svelte';

	let recentBlocks = [];
	let stats = {
		latestBlock: 0,
		totalTransactions: 0,
		activeAccounts: 0,
		tps: 0
	};
	let searchQuery = '';
	let loading = true;
	let unsubscribe;

	onMount(async () => {
		try {
			await connectToChain();

			unsubscribe = await subscribeToBlocks((block) => {
				stats.latestBlock = block.number;

				recentBlocks = [
					{
						...block,
						status: 'finalized',
						timestamp: Date.now(),
						extrinsics: []
					},
					...recentBlocks
				].slice(0, 10);

				loading = false;
			});
		} catch (error) {
			console.error('Failed to initialize explorer:', error);
			loading = false;
		}

		return () => {
			if (unsubscribe) unsubscribe();
		};
	});

	function handleSearch(e) {
		e.preventDefault();
		if (searchQuery.trim()) {
			window.location.href = `/explorer/search?q=${encodeURIComponent(searchQuery)}`;
		}
	}
</script>

<svelte:head>
	<title>VeloPay Explorer</title>
</svelte:head>

<div class="min-h-screen bg-neutral-50 dark:bg-neutral-950">
	<div class="border-b border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between mb-6">
				<div>
					<h1 class="text-3xl font-bold text-neutral-900 dark:text-white mb-2">VeloPay Explorer</h1>
					{#if $isConnected}
						<div class="flex items-center gap-2 text-sm text-neutral-600 dark:text-neutral-400">
							<div class="w-2 h-2 rounded-full bg-success-500 animate-pulse"></div>
							<span>{$chainInfo.chain} • {$chainInfo.nodeName}</span>
						</div>
					{:else}
						<div class="flex items-center gap-2 text-sm text-error-600 dark:text-error-400">
							<div class="w-2 h-2 rounded-full bg-error-500"></div>
							<span>Disconnected</span>
						</div>
					{/if}
				</div>
			</div>

			<form on:submit={handleSearch} class="relative">
				<Search class="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-neutral-400" />
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Search by block number, hash, or address..."
					class="w-full pl-12 pr-4 py-3 rounded-xl border border-neutral-300 dark:border-neutral-700
                 bg-white dark:bg-neutral-800 text-neutral-900 dark:text-white
                 focus:outline-none focus:ring-2 focus:ring-accent-500 focus:border-transparent
                 placeholder-neutral-400 dark:placeholder-neutral-500"
				/>
			</form>
		</div>
	</div>

	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
			<StatCard title="Latest Block" value="#{stats.latestBlock}" icon={Blocks} {loading} />
			<StatCard
				title="Total Transactions"
				value={stats.totalTransactions.toLocaleString()}
				icon={Activity}
				{loading}
			/>
			<StatCard
				title="Active Accounts"
				value={stats.activeAccounts.toLocaleString()}
				icon={Users}
				{loading}
			/>
			<StatCard
				title="TPS"
				value={stats.tps.toFixed(2)}
				subtitle="Transactions/sec"
				icon={TrendingUp}
				{loading}
			/>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
			<div>
				<h2 class="text-xl font-bold text-neutral-900 dark:text-white mb-4">Recent Blocks</h2>
				<div class="space-y-3">
					{#if loading}
						<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
						{#each Array(5) as _block, i (i)}
							<div class="h-24 bg-neutral-200 dark:bg-neutral-800 rounded-lg animate-pulse"></div>
						{/each}
					{:else if recentBlocks.length === 0}
						<div class="text-center py-12 text-neutral-500 dark:text-neutral-400">
							No blocks yet
						</div>
					{:else}
						{#each recentBlocks as block (block.hash)}
							<BlockCard {block} />
						{/each}
					{/if}
				</div>
			</div>

			<div>
				<h2 class="text-xl font-bold text-neutral-900 dark:text-white mb-4">Recent Transactions</h2>
				<div class="space-y-3">
					<div class="text-center py-12 text-neutral-500 dark:text-neutral-400">Coming soon</div>
				</div>
			</div>
		</div>
	</div>
</div>
