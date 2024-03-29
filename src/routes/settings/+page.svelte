<script>
	import { getUser, logout } from '$lib';
	import { onMount } from 'svelte';
	import Login from '$lib/components/Login.svelte';
	import ChangeEmail from '$lib/components/settings/ChangeEmail.svelte';
	import RemoveKrakenKeys from '$lib/components/settings/RemoveKrakenKeys.svelte';
	import ChangePassword from '$lib/components/settings/ChangePassword.svelte';
	import AddKraken from '$lib/components/settings/AddKraken.svelte';
	import Welcome from '$lib/components/settings/Welcome.svelte';

	let err = '';
	let user = '';
	let section = 'default';

	onMount(async () => {
		user = await getUser();
	});
</script>

<div class="divider divider-accent text-accent mt-0 mb-3">Settings</div>
{#if user}
	<div class="flex space-x-4 m-0" style="height: 42rem;">
		<ul class="menu text-accent bg-base-200 w-60">
			<div class="flex max-w-xs justify-between rounded-full bg-gray-800 text-sm">
				<div class="h-8 w-8 rounded-full border border-primary">
					<minidenticon-svg username={user.username}></minidenticon-svg>
				</div>
				<h2 class="font-bold text-primary text-lg pr-2">
					{user.username.charAt(0).toUpperCase() + user.username.slice(1)}
				</h2>
			</div>
			<li class="menu-title text-primary">User</li>
			<li>
				<a href="/settings" on:click={() => (section = 'change_password')}>Change Password</a>
			</li>
			<!-- <li>
				 <a href="/settings" on:click={() => (section = 'change_email')}>Change Email</a>
				 </li> -->
			<li>
				<button on:click={logout}>Logout</button>
			</li>
			<li class="menu-title text-primary mt-4">Trading</li>
			{#if user.kraken_auth}
				<li>
					<a href="/settings" on:click={() => (section = 'rm_kraken_keys')}>- Kraken API Keys</a>
				</li>
			{:else}
				<li>
					<a href="/settings" on:click={() => (section = 'add_kraken')}>+ Kraken API Keys</a>
				</li>
			{/if}
			{#if user.ash_auth}
				<li><a href="/">- Ash API Keys</a></li>
			{:else}
				<li><a href="/addAsh">+ Ash API Keys</a></li>
			{/if}
		</ul>
		{#if section == 'change_email'}
			<ChangeEmail {err} />
		{:else if section == 'change_password'}
			<ChangePassword {err} />
		{:else if section == 'add_kraken'}
			<AddKraken {err} />
		{:else if section == 'rm_kraken_keys'}
			<RemoveKrakenKeys {err} />
		{:else}
			<Welcome msg="" clear="" />
		{/if}
	</div>
{:else}
	<Login uName="" msg="" err="" />
{/if}
