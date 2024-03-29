<script>
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { clearUserStore, ok, storeParams, storeUser } from '$lib';
	import Loading from '$lib/components/Loading.svelte';
	import Unlock from '$lib/components/Unlock.svelte';

	export let password;
	export let user;
	export let msg;

	let promise = invoke('unlock', { password });

	onMount(async () => {
		if (user) {
			clearUserStore();
			storeUser(user);
		}
		if (msg) {
			ok(msg);
		}
	});
</script>

{#await promise}
	<Loading />
{:then params}
	{storeParams(params)}
	{goto('/', {
		invalidate: ['/']
	})}
{:catch err}
	<Unlock {err} />
{/await}
