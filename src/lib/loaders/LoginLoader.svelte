<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import Loading from '$lib/components/Loading.svelte';
	import Login from '$lib/components/Login.svelte';
	import UnlockLoader from '$lib/loaders/UnlockLoader.svelte';

	export let username;
	export let password;

	let uName = username;
	let msg = '';

	let promise = invoke('login', { username, password });
</script>

{#await promise}
	<Loading />
{:then user}
	<UnlockLoader {password} {user} msg="" />
{:catch err}
	<Login {uName} {msg} {err} />
{/await}
