<script>
	import { confirm } from '@tauri-apps/api/dialog';
	import { addOrder } from '$lib';
	import KrakenImg from '$lib/icons/kraken.png';

	export let data;
	export let asset;
	export let portfolio;

	let sellAmt = 0;
	let sellAmtQuote = 0;
	let baseAmt = true;
	let price = data[data.length - 1].close;
	let maxQuote = (price * getAvailableBase()).toFixed(asset.quote_decimals);
	let disclaimer =
		'Note: This executes "Market" orders only. For other order types, please use Kraken website.';

	async function submit() {
		let q;
		let amt;
		if (baseAmt) {
			if (sellAmt == 0) {
				return;
			}
			amt = sellAmt;
			let quoteEstimate = sellAmt * price;
			q = `Sell ${sellAmt.toFixed(asset.base_decimals)} ${asset.base} for ~${quoteEstimate.toFixed(asset.quote_decimals)} ${asset.quote} at market price (~${price} ${asset.base}/${asset.quote})?\n\n${disclaimer}`;
		} else {
			if (sellAmtQuote == 0) {
				return;
			}
			amt = sellAmtQuote / price;
			q = `Sell ${amt.toFixed(asset.base_decimals)} ${asset.base} for ~${sellAmtQuote.toFixed(asset.quote_decimals)} ${asset.quote} at market price (~${price} ${asset.base}/${asset.quote})?\n\n${disclaimer}`;
		}
		const conf = await confirm(q, {
			title: `Sell ${asset.base}`,
			type: 'warning'
		});
		if (conf) {
			await addOrder(asset.quote, asset.base, 'sell', amt);
			console.log('submit transaction');
		}
	}

	function getAvailableBase() {
		if (portfolio) {
			let quote = portfolio.find((a) => a.name == asset.base);
			if (quote) {
				return quote.balance;
			}
		}
		return 0;
	}
</script>

<div class="card w-96 h-64 bg-neutral text-error ring ring-error">
	{#if getAvailableBase() > 0}
		<div class="card-body items-center text-center pt-3 px-2">
			<h2 class="card-title">Sell {asset.base}</h2>
			<div class="flex items-center gap-1 pt-1">
				<h4 class="text-sm">
					Available: {getAvailableBase().toFixed(asset.base_decimals)}
					{asset.base}
				</h4>
			</div>
			<div class="flex items-center gap-1 pt-1">
				<label class="form-control w-80 max-w-xs space-y-1">
					{#if baseAmt}
						<input
							type="number"
							bind:value={sellAmt}
							min="0"
							max={getAvailableBase()}
							placeholder="Sell Amount ({asset.base})"
							step={asset.ordermin}
							class="input input-bordered input-error w-full max-w-xs"
						/>
						<input
							type="range"
							bind:value={sellAmt}
							min="0"
							max={getAvailableBase()}
							step={asset.ordermin}
							class="range range-error range-xs"
						/>
					{:else}
						<input
							type="number"
							bind:value={sellAmtQuote}
							min="0"
							max={maxQuote}
							step={asset.costmin}
							placeholder="Sell Amount ({asset.quote})"
							class="input input-bordered input-error w-full max-w-xs"
						/>
						<input
							type="range"
							bind:value={sellAmtQuote}
							min="0"
							max={maxQuote}
							step={asset.costmin}
							class="range range-error range-xs"
						/>
					{/if}
				</label>
				<label class="swap swap-rotate">
					<input type="checkbox" bind:checked={baseAmt} />
					<div class="skeleton w-10 h-10 rounded-full mb-5 ml-1"></div>
					<div class="swap-on rounded-full w-6 h-6 mt-2 ml-3">
						<img
							src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{asset.base.toLowerCase()}.webp"
							alt={KrakenImg}
						/>
					</div>
					<div class="swap-off rounded-full w-6 h-6 mt-2 ml-3 z-10">
						<img
							src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{asset.quote.toLowerCase()}.webp"
							alt={KrakenImg}
						/>
					</div>
				</label>
			</div>
			{#if baseAmt}
				<p class="text-xs">
					Sell {sellAmt}
					{asset.base} for ~{sellAmt * price}
					{asset.quote}
				</p>
			{:else}
				<p class="text-xs">
					Sell {sellAmtQuote / price}
					{asset.base} for ~{sellAmtQuote}
					{asset.quote}
				</p>
			{/if}
			<div class="card-actions justify-end">
				<button class="btn btn-error" on:click={submit}>Submit</button>
			</div>
		</div>
	{:else}
		<div class="card-body items-center text-center pt-3 px-2">
			<h2 class="card-title">Sell {asset.base}</h2>
			<div class="flex items-center gap-1 pt-1">
				<h4 class="text-sm">
					No {asset.base} available.
				</h4>
			</div>
		</div>
	{/if}
</div>
