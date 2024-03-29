<script>
	import { clearUserStore, getUser, ok } from '$lib';
	import { onMount } from 'svelte';

	export let msg;
	export let clear;

	let user = '';

	onMount(async () => {
		user = await getUser();
		if (clear) {
			clearUserStore();
		}
		if (msg) {
			await ok(msg);
		}
	});
</script>

{#if user}
	<div class="hero min-h bg-base-200">
		<div class="hero-content text-center text-primary">
			<div class="max-w-md">
				<h1 class="text-5xl font-bold">
					Welcome {user.username.charAt(0).toUpperCase() + user.username.slice(1)}
				</h1>
			</div>
		</div>
	</div>
{/if}
