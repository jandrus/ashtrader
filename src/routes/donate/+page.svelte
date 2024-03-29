<script>
	import XMR from '$lib/icons/xmr.png';
	import BTC from '$lib/icons/btc.png';
	import KrakenImg from '$lib/icons/kraken.png';

	let section = 'btc';
</script>

<div class="divider divider-accent text-accent my-0">
	Donate
	<div class="w-12 h-12 mt-7">
		<img
			src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{section}.webp"
			alt={KrakenImg}
		/>
	</div>
</div>
<div class="carousel w-full" style="height: 36rem;">
	<div id="btc" class="carousel-item relative w-full">
		<div class="card-body items-center text-center pt-3 px-2">
			<img src={BTC} alt="Bitcoin address" style="min-width: 100px; max-width: 420px;" />
			<div class="flex items-center gap-1">
				<div>
					<div class="font-bold">
						<p class="text-sm text-accent underline">BTC Address:</p>
						<p class="text-sm text-accent">bc1q7y20wr2n5qt2fxe569llvz5a0qsnpsz4decplr</p>
					</div>
				</div>
			</div>
			<div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
				<a href="#xmr" class="btn btn-circle" on:click={() => (section = 'xmr')}>❮</a>
				<a href="#xmr" class="btn btn-circle" on:click={() => (section = 'xmr')}>❯</a>
			</div>
		</div>
	</div>
	<div id="xmr" class="carousel-item relative w-full">
		<div class="card-body items-center text-center pt-3 px-2">
			<img src={XMR} alt="Monero address" style="min-width: 100px; max-width: 420px;" />
			<div class="flex items-center gap-1">
				<div>
					<div class="font-bold">
						<p class="text-sm text-accent underline">XMR Address:</p>
						<p class="text-sm text-accent">
							84t9GUWQVJSGxF8cbMtRBd67YDAHnTsrdWVStcdpiwcAcAnVy21U6RmLdwiQdbfsyu16UqZn6qj1gGheTMkHkYA4HbVN4zS
						</p>
					</div>
				</div>
			</div>
			<div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
				<a href="#btc" class="btn btn-circle" on:click={() => (section = 'btc')}>❮</a>
				<a href="#btc" class="btn btn-circle" on:click={() => (section = 'btc')}>❯</a>
			</div>
		</div>
	</div>
</div>
<div class="flex justify-center w-full py-2 gap-2">
	{#if section == 'btc'}
		<a
			href="#btc"
			class="btn btn-active btn-outline btn-primary btn-xs"
			on:click={() => (section = 'btc')}>Bitcoin</a
		>
	{:else}
		<a href="#btc" class="btn btn-outline btn-primary btn-xs" on:click={() => (section = 'btc')}
			>Bitcoin</a
		>
	{/if}
	{#if section == 'xmr'}
		<a
			href="#xmr"
			class="btn btn-active btn-outline btn-primary btn-xs"
			on:click={() => (section = 'xmr')}>Monero</a
		>
	{:else}
		<a href="#xmr" class="btn btn-outline btn-primary btn-xs" on:click={() => (section = 'xmr')}
			>Monero</a
		>
	{/if}
</div>
<div class="card-body items-center text-center text-primary pt-3 px-2 flex">
	We appreciate your support.
</div>
