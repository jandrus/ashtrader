<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import AddKrakenKey from '$lib/components/AddKrakenKey.svelte';
	import Loading from '$lib/components/Loading.svelte';
	import UnlockLoader from '$lib/loaders/UnlockLoader.svelte';

	export let user;
	export let krakenApi;
	export let krakenSk;
	export let password;

	let promise = invoke('add_kraken', { krakenApi, krakenSk, password });
</script>

{#await promise}
	<Loading />
{:then}
	<UnlockLoader {password} {user} msg="Kraken API Keys added" />
{:catch error}
	<AddKrakenKey err={error} />
{/await}
