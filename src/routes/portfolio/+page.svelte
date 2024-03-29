<script>
	import { getUser, unlocked } from '$lib';
	import { onMount } from 'svelte';
	import Login from '$lib/components/Login.svelte';
	import AddKrakenKey from '$lib/components/AddKrakenKey.svelte';
	import Portfolio from '$lib/components/Portfolio.svelte';
	import Unlock from '$lib/components/Unlock.svelte';

	let user;
	let isUnlocked;

	onMount(async () => {
		user = await getUser();
		isUnlocked = await unlocked();
	});
</script>

{#if user}
	{#if user.kraken_auth}
		{#if isUnlocked}
			<Portfolio />
		{:else}
			<Unlock err="" />
		{/if}
	{:else}
		<AddKrakenKey err="" />
	{/if}
{:else}
	<Login uName="" msg="" err="" />
{/if}
