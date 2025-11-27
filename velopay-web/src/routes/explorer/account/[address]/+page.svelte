<script>
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { connectToChain, getAccount } from '$lib/explorer/api/chainApi';
	import Card from '$lib/explorer/components/Card.svelte';
	import { ArrowLeft, Wallet, DollarSign, Lock, Hash } from 'lucide-svelte';
	import { goto } from '$app/navigation';

	let accountData = null;
	let loading = true;
	let error = null;

	$: address = $page.params.address;

	onMount(async () => {
		try {
			await connectToChain();
			const account = await getAccount(address);
			accountData = account;
			loading = false;
		} catch (err) {
			error = err.message;
			loading = false;
		}
	});

	function formatBalance(balance) {
		const num = parseInt(balance) / 1e12;
		return num.toLocaleString(undefined, { minimumFractionDigits: 4, maximumFractionDigits: 4 });
	}

	function copyToClipboard(text) {
		navigator.clipboard.writeText(text);
	}
</script>

<svelte:head>
	<title>Account {address.slice(0, 8)}... - Velo Chain Explorer</title>
</svelte:head>

<div class="min-h-screen bg-neutral-50 dark:bg-neutral-950">
	<div class="border-b border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<button
				on:click={() => goto('/explorer')}
				class="flex items-center gap-2 text-neutral-600 dark:text-neutral-400 hover:text-accent-600
               dark:hover:text-accent-400 transition-colors mb-4"
			>
				<ArrowLeft class="w-4 h-4" />
				Back to Explorer
			</button>
			<div class="flex items-center gap-3">
				<div class="p-3 rounded-xl bg-accent-50 dark:bg-accent-950">
					<Wallet class="w-6 h-6 text-accent-600 dark:text-accent-400" />
				</div>
				<div>
					<h1 class="text-2xl font-bold text-neutral-900 dark:text-white">Account Details</h1>
					<p class="text-sm text-neutral-600 dark:text-neutral-400 font-mono">
						{address.slice(0, 12)}...{address.slice(-12)}
					</p>
				</div>
			</div>
		</div>
	</div>

	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		{#if loading}
			<div class="space-y-6">
				<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
				{#each Array(3) as _skeleton, i (i)}
					<div class="h-48 bg-neutral-200 dark:bg-neutral-800 rounded-xl animate-pulse"></div>
				{/each}
			</div>
		{:else if error}
			<Card>
				<div class="p-8 text-center">
					<h2 class="text-xl font-bold text-neutral-900 dark:text-white mb-2">
						Error Loading Account
					</h2>
					<p class="text-neutral-600 dark:text-neutral-400">
						{error}
					</p>
				</div>
			</Card>
		{:else if accountData}
			<div class="space-y-6">
				<Card>
					<div class="p-6">
						<h2 class="text-lg font-bold text-neutral-900 dark:text-white mb-6">
							Account Information
						</h2>
						<div class="space-y-4">
							<div class="border-b border-neutral-200 dark:border-neutral-800 pb-4">
								<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-2">Address</div>
								<button
									on:click={() => copyToClipboard(address)}
									class="w-full flex items-center justify-between p-3 rounded-lg bg-neutral-50
                         dark:bg-neutral-800 hover:bg-neutral-100 dark:hover:bg-neutral-700
                         transition-colors group"
								>
									<span class="font-mono text-sm text-neutral-900 dark:text-white break-all">
										{address}
									</span>
									<Hash
										class="w-4 h-4 text-neutral-400 group-hover:text-accent-600 flex-shrink-0 ml-2"
									/>
								</button>
							</div>

							<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
								<div>
									<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-1">Nonce</div>
									<div class="text-2xl font-bold text-neutral-900 dark:text-white">
										{accountData.nonce}
									</div>
								</div>
								<div>
									<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-1">Providers</div>
									<div class="text-2xl font-bold text-neutral-900 dark:text-white">
										{accountData.providers}
									</div>
								</div>
							</div>
						</div>
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<h2
							class="text-lg font-bold text-neutral-900 dark:text-white mb-6 flex items-center gap-2"
						>
							<DollarSign class="w-5 h-5" />
							Balances
						</h2>
						<div class="space-y-4">
							<div
								class="p-4 rounded-lg bg-gradient-to-br from-accent-50 to-primary-50
                          dark:from-accent-950 dark:to-primary-950 border border-accent-200
                          dark:border-accent-800"
							>
								<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-2">Free Balance</div>
								<div class="text-3xl font-bold text-accent-600 dark:text-accent-400">
									{formatBalance(accountData.data.free)} VCS
								</div>
							</div>

							<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
								<div
									class="p-4 rounded-lg bg-neutral-50 dark:bg-neutral-800 border border-neutral-200
                            dark:border-neutral-700"
								>
									<div
										class="flex items-center gap-2 text-sm text-neutral-600 dark:text-neutral-400 mb-2"
									>
										<Lock class="w-4 h-4" />
										Reserved
									</div>
									<div class="text-xl font-bold text-neutral-900 dark:text-white">
										{formatBalance(accountData.data.reserved)} VCS
									</div>
								</div>

								<div
									class="p-4 rounded-lg bg-neutral-50 dark:bg-neutral-800 border border-neutral-200
                            dark:border-neutral-700"
								>
									<div
										class="flex items-center gap-2 text-sm text-neutral-600 dark:text-neutral-400 mb-2"
									>
										<Lock class="w-4 h-4" />
										Frozen
									</div>
									<div class="text-xl font-bold text-neutral-900 dark:text-white">
										{formatBalance(accountData.data.frozen)} VCS
									</div>
								</div>
							</div>

							<div
								class="p-4 rounded-lg bg-success-50 dark:bg-success-950 border border-success-200
                          dark:border-success-800"
							>
								<div class="text-sm text-success-700 dark:text-success-400 mb-2">Total Balance</div>
								<div class="text-2xl font-bold text-success-700 dark:text-success-400">
									{formatBalance(
										(
											parseInt(accountData.data.free) + parseInt(accountData.data.reserved)
										).toString()
									)} VCS
								</div>
							</div>
						</div>
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<h2 class="text-lg font-bold text-neutral-900 dark:text-white mb-4">
							Transaction History
						</h2>
						<div class="text-center py-12 text-neutral-500 dark:text-neutral-400">Coming soon</div>
					</div>
				</Card>
			</div>
		{/if}
	</div>
</div>
