<script>
	import { confirm } from '@tauri-apps/api/dialog';
	import { addOrder } from '$lib';
	import KrakenImg from '$lib/icons/kraken.png';

	export let data;
	export let asset;
	export let portfolio;

	let buyAmt = 0;
	let buyAmtBase = 0;
	let quoteAmt = true;
	let price = data[data.length - 1].close;
	let maxBase = (getAvailableQuote() / price).toFixed(asset.base_decimals);
	let disclaimer =
		'Note: This executes "Market" orders only. For other order types, please use Kraken website.';

	async function submit() {
		let q;
		let amt;
		if (quoteAmt) {
			if (buyAmt == 0) {
				return;
			}
			amt = buyAmt;
			let baseEstimate = buyAmt / price;
			q = `Buy ${baseEstimate.toFixed(asset.base_decimals)} ${asset.base} for ~${buyAmt.toFixed(asset.quote_decimals)} ${asset.quote} at market price (${price} ${asset.base}/${asset.quote})?\n\n${disclaimer}`;
		} else {
			if (buyAmtBase == 0) {
				return;
			}
			amt = buyAmtBase * price;
			q = `Buy ${buyAmtBase.toFixed(asset.base_decimals)} ${asset.base} for ~${amt.toFixed(asset.quote_decimals)} ${asset.quote} at market price (${price} ${asset.base}/${asset.quote})?\n\n${disclaimer}`;
		}
		const conf = await confirm(q, {
			title: `Buy ${asset.base}`,
			type: 'warning'
		});
		if (conf) {
			await addOrder(asset.quote, asset.base, 'buy', amt);
			console.log('submit transaction');
		}
	}

	function getAvailableQuote() {
		if (portfolio) {
			let quote = portfolio.find((a) => a.name == asset.quote);
			if (quote) {
				return quote.balance;
			}
		}
		return 0;
	}
</script>

<div class="card w-96 h-64 bg-neutral text-success ring ring-success">
	{#if getAvailableQuote() > 0}
		<div class="card-body items-center text-center pt-3 px-2">
			<h2 class="card-title">Buy {asset.base}</h2>
			<div class="flex items-center gap-1 pt-1">
				<h4 class="text-sm">
					Available: {getAvailableQuote().toFixed(asset.quote_decimals)}
					{asset.quote}
				</h4>
			</div>
			<div class="flex items-center gap-1 pt-1">
				<label class="form-control w-80 max-w-xs space-y-1">
					{#if quoteAmt}
						<input
							type="number"
							bind:value={buyAmt}
							min="0"
							max={getAvailableQuote()}
							step={asset.costmin}
							placeholder="Sell Amount ({asset.quote})"
							class="input input-bordered input-success w-full max-w-xs"
						/>
						<input
							type="range"
							bind:value={buyAmt}
							min="0"
							max={getAvailableQuote()}
							step={asset.costmin}
							class="range range-success range-xs"
						/>
					{:else}
						<input
							type="number"
							bind:value={buyAmtBase}
							min="0"
							max={maxBase}
							step={asset.ordermin}
							placeholder="Sell Amount ({asset.base})"
							class="input input-bordered input-success w-full max-w-xs"
						/>
						<input
							type="range"
							bind:value={buyAmtBase}
							min="0"
							max={maxBase}
							step={asset.ordermin}
							class="range range-success range-xs"
						/>
					{/if}
				</label>
				<label class="swap swap-rotate">
					<input type="checkbox" bind:checked={quoteAmt} />
					<div class="skeleton w-10 h-10 rounded-full mb-5 ml-1"></div>
					<div class="swap-on rounded-full w-6 h-6 mt-2 ml-3">
						<img
							src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{asset.quote.toLowerCase()}.webp"
							alt={KrakenImg}
						/>
					</div>
					<div class="swap-off rounded-full w-6 h-6 mt-2 ml-3 z-10">
						<img
							src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{asset.base.toLowerCase()}.webp"
							alt={KrakenImg}
						/>
					</div>
				</label>
			</div>
			{#if quoteAmt}
				<p class="text-xs">
					Buy {buyAmt / price}
					{asset.base} for ~{buyAmt}
					{asset.quote}
				</p>
			{:else}
				<p class="text-xs">
					Buy {buyAmtBase}
					{asset.base} for ~{buyAmtBase * price}
					{asset.quote}
				</p>
			{/if}
			<div class="card-actions justify-end">
				<button class="btn btn-success" on:click={submit}>Submit</button>
			</div>
		</div>
	{:else}
		<div class="card-body items-center text-center pt-3 px-2">
			<h2 class="card-title">Buy {asset.base}</h2>
			<div class="flex items-center gap-1 pt-1">
				<h4 class="text-sm">
					No {asset.quote} available.
				</h4>
			</div>
		</div>
	{/if}
</div>
