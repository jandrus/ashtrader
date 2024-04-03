<script>
	import '$lib/css/app.css';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { themeChange } from 'theme-change';
	import { onMount } from 'svelte';

	document.addEventListener('contextmenu', (event) => event.preventDefault());
	onMount(() => themeChange());
</script>

<main class="bg-primary-content">
	<Nav />
	<slot />
	<Footer />
</main>
