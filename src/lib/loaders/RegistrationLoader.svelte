<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import Loading from '$lib/components/Loading.svelte';
	import Login from '$lib/components/Login.svelte';
	import Registration from '$lib/components/Registration.svelte';

	export let username;
	export let email;
	export let password;

	let uName = username;
	let err = '';

	let promise = invoke('register', { username, email, password });
</script>

{#await promise}
	<Loading />
{:then msg}
	<Login {uName} {msg} {err} />
{:catch err}
	<Registration {err} />
{/await}
