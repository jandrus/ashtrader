<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Loading from '$lib/components/Loading.svelte';
	import Table from '$lib/components/Table.svelte';

	let promise = invoke('get_trading_pairs');

	onMount(async () => {
		promise = invoke('get_trading_pairs');
	});
</script>

{#await promise}
	<Loading />
{:then tradingPairs}
	<Table {tradingPairs} />
{:catch err}
	<p class="text-red-600">{err}</p>
{/await}
