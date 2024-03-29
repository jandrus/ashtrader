<script>
	import { onMount } from 'svelte';
	import { getSelectedMarket } from '$lib';
	import MarketLoader from '$lib/loaders/MarketLoader.svelte';

	let market;

	onMount(async () => {
		market = await getSelectedMarket();
	});
</script>

<MarketLoader {market} />
