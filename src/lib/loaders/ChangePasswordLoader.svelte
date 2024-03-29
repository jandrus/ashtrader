<script>
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/tauri';
	import { clearUserStore } from '$lib';
	import ChangePassword from '$lib/components/settings/ChangePassword.svelte';
	import LoadingSettings from '$lib/components/settings/LoadingSettings.svelte';

	export let oldPassword;
	export let newPassword;

	let promise = invoke('change_password', { oldPassword, newPassword });
</script>

{#await promise}
	<LoadingSettings />
{:then}
	{clearUserStore()}
	{goto('/login', {
		invalidateAll: true
	})}
{:catch err}
	<ChangePassword {err} />
{/await}
