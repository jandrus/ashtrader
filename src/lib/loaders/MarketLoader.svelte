<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';
	import { err } from '$lib';
	import Loading from '$lib/components/Loading.svelte';
	import Market from '$lib/components/Market.svelte';

	export let market;

	let promise = invoke('get_trading_pairs');
</script>

{#await promise}
	<Loading />
{:then tradingPairs}
	<Market {tradingPairs} {market} />
{:catch error}
	{err(error)}
	{goto('/', {
		invalidate: ['/']
	})}
{/await}
