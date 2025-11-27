import { ApiPromise, WsProvider } from '@polkadot/api';
import { writable, get } from 'svelte/store';

const CHAIN_ENDPOINT = 'ws://127.0.0.1:9944';

export const api = writable(null);
export const isConnected = writable(false);
export const chainInfo = writable({
	chain: '',
	nodeName: '',
	nodeVersion: ''
});

export async function connectToChain() {
	try {
		const wsProvider = new WsProvider(CHAIN_ENDPOINT);
		const apiInstance = await ApiPromise.create({ provider: wsProvider });

		api.set(apiInstance);
		isConnected.set(true);

		const [chain, nodeName, nodeVersion] = await Promise.all([
			apiInstance.rpc.system.chain(),
			apiInstance.rpc.system.name(),
			apiInstance.rpc.system.version()
		]);

		chainInfo.set({
			chain: chain.toString(),
			nodeName: nodeName.toString(),
			nodeVersion: nodeVersion.toString()
		});

		return apiInstance;
	} catch (error) {
		console.error('Failed to connect to chain:', error);
		isConnected.set(false);
		throw error;
	}
}

export async function subscribeToBlocks(callback) {
	const apiInstance = get(api);
	if (!apiInstance) throw new Error('API not connected');

	return apiInstance.rpc.chain.subscribeNewHeads((header) => {
		callback({
			number: header.number.toNumber(),
			hash: header.hash.toHex(),
			parentHash: header.parentHash.toHex(),
			stateRoot: header.stateRoot.toHex(),
			extrinsicsRoot: header.extrinsicsRoot.toHex()
		});
	});
}

export async function getBlock(hashOrNumber) {
	const apiInstance = get(api);
	if (!apiInstance) throw new Error('API not connected');

	const hash =
		typeof hashOrNumber === 'number'
			? await apiInstance.rpc.chain.getBlockHash(hashOrNumber)
			: hashOrNumber;

	const signedBlock = await apiInstance.rpc.chain.getBlock(hash);
	const block = signedBlock.block;

	return {
		number: block.header.number.toNumber(),
		hash: hash.toHex(),
		parentHash: block.header.parentHash.toHex(),
		stateRoot: block.header.stateRoot.toHex(),
		extrinsicsRoot: block.header.extrinsicsRoot.toHex(),
		extrinsics: block.extrinsics.map((ext, index) => ({
			index,
			method: ext.method.method,
			section: ext.method.section,
			hash: ext.hash.toHex(),
			isSigned: ext.isSigned,
			signer: ext.isSigned ? ext.signer.toString() : null
		}))
	};
}

export async function getAccount(address) {
	const apiInstance = get(api);
	if (!apiInstance) throw new Error('API not connected');

	const account = await apiInstance.query.system.account(address);

	return {
		nonce: account.nonce.toNumber(),
		consumers: account.consumers.toNumber(),
		providers: account.providers.toNumber(),
		sufficients: account.sufficients.toNumber(),
		data: {
			free: account.data.free.toString(),
			reserved: account.data.reserved.toString(),
			frozen: account.data.frozen.toString()
		}
	};
}

export async function searchByHash(hash) {
	const apiInstance = get(api);
	if (!apiInstance) throw new Error('API not connected');

	try {
		const block = await apiInstance.rpc.chain.getBlock(hash);
		return { type: 'block', data: block };
	} catch {
		try {
			const header = await apiInstance.rpc.chain.getHeader(hash);
			return { type: 'block', data: { header } };
		} catch {
			return { type: 'unknown', data: null };
		}
	}
}
