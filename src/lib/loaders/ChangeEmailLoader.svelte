<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import Welcome from '$lib/components/settings/Welcome.svelte';
	import ChangeEmail from '$lib/components/settings/ChangeEmail.svelte';

	export let password;
	export let email;

	async function changeEmail() {
		return invoke('change_email', { password, email });
	}

	let msg = 'Email changed successfully. Please verify new address.';
	let promise = changeEmail();
</script>

{#await promise}
	<div class="hero min-h bg-base-200">
		<div class="hero-content text-center text-primary">
			<div class="max-w-md">
				<span class="loading loading-infinity loading-lg"></span>
			</div>
		</div>
	</div>
{:then}
	<Welcome {msg} clear="true" />
{:catch err}
	<ChangeEmail {err} />
{/await}
