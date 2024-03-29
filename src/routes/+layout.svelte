<script>
	import '$lib/css/app.css';
	import Nav from '$lib/components/Nav.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>

<main class="bg-primary-content">
	<Nav />
	<slot />
	<Footer />
</main>
