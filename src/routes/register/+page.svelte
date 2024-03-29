<script>
	import { onMount } from 'svelte';
	import { err, getUser } from '$lib';
	import { goto } from '$app/navigation';
	import Registration from '$lib/components/Registration.svelte';

	onMount(async () => {
		let user = await getUser();
		if (user) {
			await err('Please logout first');
			goto('/').then(() => goto('/'));
		}
	});
</script>

<Registration err="" />
