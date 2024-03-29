<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import Welcome from '$lib/components/settings/Welcome.svelte';
	import RemoveKrakenKeys from '$lib/components/settings/RemoveKrakenKeys.svelte';
	import LoadingSettings from '$lib/components/settings/LoadingSettings.svelte';

	export let password;

	let msg = 'Kraken keys removed. Please add keys to trade.';

	let promise = invoke('remove_kraken_keys', { password });
</script>

{#await promise}
	<LoadingSettings />
{:then}
	<Welcome {msg} clear="true" />
{:catch err}
	<RemoveKrakenKeys {err} />
{/await}
