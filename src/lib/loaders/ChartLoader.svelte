<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import LineChart from '$lib/components/plot/LineChart.svelte';
	import Loading from '$lib/components/Loading.svelte';

	export let market;
	export let asset;
	export let interval;

	let tmpInterval = interval;

	let promise = invoke('data', { market, interval });

	onMount(() => {
		const interval = setInterval(() => {
			promise = invoke('data', { market, interval: tmpInterval });
		}, 60000);
		return () => {
			clearInterval(interval);
		};
	});
</script>

{#await promise}
	<Loading />
{:then data}
	<LineChart {data} {asset} interval={tmpInterval} />
{:catch err}
	<p class="text-red-600">{err}</p>
{/await}
