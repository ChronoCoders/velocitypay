<script>
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { connectToChain, getBlock } from '$lib/explorer/api/chainApi';
	import Card from '$lib/explorer/components/Card.svelte';
	import { ArrowLeft, Cube, Hash, FileText, CheckCircle, XCircle } from 'lucide-svelte';
	import { goto } from '$app/navigation';

	let blockData = null;
	let loading = true;
	let error = null;

	$: blockId = $page.params.id;

	onMount(async () => {
		try {
			await connectToChain();
			const block = await getBlock(parseInt(blockId) || blockId);
			blockData = block;
			loading = false;
		} catch (err) {
			error = err.message;
			loading = false;
		}
	});

	function truncateHash(hash, start = 16, end = 16) {
		return `${hash.slice(0, start)}...${hash.slice(-end)}`;
	}

	function copyToClipboard(text) {
		navigator.clipboard.writeText(text);
	}
</script>

<svelte:head>
	<title>Block #{blockId} - VeloPay Explorer</title>
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
				<div class="p-3 rounded-xl bg-primary-50 dark:bg-primary-950">
					<Cube class="w-6 h-6 text-primary-600 dark:text-primary-400" />
				</div>
				<div>
					<h1 class="text-2xl font-bold text-neutral-900 dark:text-white">
						Block #{blockId}
					</h1>
					<p class="text-sm text-neutral-600 dark:text-neutral-400">Block details and extrinsics</p>
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
					<XCircle class="w-12 h-12 text-error-500 mx-auto mb-4" />
					<h2 class="text-xl font-bold text-neutral-900 dark:text-white mb-2">Block Not Found</h2>
					<p class="text-neutral-600 dark:text-neutral-400">
						{error}
					</p>
				</div>
			</Card>
		{:else if blockData}
			<div class="space-y-6">
				<Card>
					<div class="p-6">
						<h2 class="text-lg font-bold text-neutral-900 dark:text-white mb-6">
							Block Information
						</h2>
						<div class="space-y-4">
							<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
								<div>
									<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-1">
										Block Number
									</div>
									<div class="text-lg font-mono font-semibold text-neutral-900 dark:text-white">
										#{blockData.number}
									</div>
								</div>
								<div>
									<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-1">Extrinsics</div>
									<div class="text-lg font-semibold text-neutral-900 dark:text-white">
										{blockData.extrinsics.length}
									</div>
								</div>
							</div>

							<div class="border-t border-neutral-200 dark:border-neutral-800 pt-4">
								<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-2">Block Hash</div>
								<button
									on:click={() => copyToClipboard(blockData.hash)}
									class="w-full flex items-center justify-between p-3 rounded-lg bg-neutral-50
                         dark:bg-neutral-800 hover:bg-neutral-100 dark:hover:bg-neutral-700
                         transition-colors group"
								>
									<span class="font-mono text-sm text-neutral-900 dark:text-white">
										{blockData.hash}
									</span>
									<Hash class="w-4 h-4 text-neutral-400 group-hover:text-accent-600" />
								</button>
							</div>

							<div class="border-t border-neutral-200 dark:border-neutral-800 pt-4">
								<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-2">Parent Hash</div>
								<button
									on:click={() => copyToClipboard(blockData.parentHash)}
									class="w-full flex items-center justify-between p-3 rounded-lg bg-neutral-50
                         dark:bg-neutral-800 hover:bg-neutral-100 dark:hover:bg-neutral-700
                         transition-colors group"
								>
									<span class="font-mono text-sm text-neutral-900 dark:text-white">
										{blockData.parentHash}
									</span>
									<Hash class="w-4 h-4 text-neutral-400 group-hover:text-accent-600" />
								</button>
							</div>

							<div
								class="grid grid-cols-1 md:grid-cols-2 gap-4 border-t border-neutral-200 dark:border-neutral-800 pt-4"
							>
								<div>
									<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-2">State Root</div>
									<div class="font-mono text-xs text-neutral-700 dark:text-neutral-300 break-all">
										{truncateHash(blockData.stateRoot)}
									</div>
								</div>
								<div>
									<div class="text-sm text-neutral-600 dark:text-neutral-400 mb-2">
										Extrinsics Root
									</div>
									<div class="font-mono text-xs text-neutral-700 dark:text-neutral-300 break-all">
										{truncateHash(blockData.extrinsicsRoot)}
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
							<FileText class="w-5 h-5" />
							Extrinsics ({blockData.extrinsics.length})
						</h2>
						<div class="space-y-3">
							{#each blockData.extrinsics as extrinsic, i (extrinsic.hash)}
								<div
									class="p-4 rounded-lg bg-neutral-50 dark:bg-neutral-800 border border-neutral-200
                            dark:border-neutral-700 hover:border-accent-500 dark:hover:border-accent-500
                            transition-colors"
								>
									<div class="flex items-start justify-between mb-3">
										<div class="flex items-center gap-3">
											<div
												class="flex items-center justify-center w-8 h-8 rounded-full
                                  bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-400
                                  text-sm font-bold"
											>
												{i}
											</div>
											<div>
												<div class="font-mono font-semibold text-neutral-900 dark:text-white">
													{extrinsic.section}.{extrinsic.method}
												</div>
												{#if extrinsic.signer}
													<div class="text-xs text-neutral-600 dark:text-neutral-400 font-mono">
														{truncateHash(extrinsic.signer, 8, 8)}
													</div>
												{/if}
											</div>
										</div>
										<div class="flex items-center gap-2">
											{#if extrinsic.isSigned}
												<span
													class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs
                                     font-medium bg-success-50 text-success-700 dark:bg-success-950
                                     dark:text-success-400"
												>
													<CheckCircle class="w-3 h-3" />
													Signed
												</span>
											{:else}
												<span
													class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs
                                     font-medium bg-neutral-100 text-neutral-600 dark:bg-neutral-700
                                     dark:text-neutral-400"
												>
													Unsigned
												</span>
											{/if}
										</div>
									</div>
									<div class="text-xs font-mono text-neutral-500 dark:text-neutral-400">
										{truncateHash(extrinsic.hash, 12, 12)}
									</div>
								</div>
							{/each}
						</div>
					</div>
				</Card>
			</div>
		{/if}
	</div>
</div>
