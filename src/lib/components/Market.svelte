<script>
	import { viewMarket } from '$lib';
	import ChartLoader from '$lib/loaders/ChartLoader.svelte';
	import KrakenImg from '$lib/icons/kraken.png';

	export let tradingPairs;
	export let market;

	function getMarket(exchangeName) {
		const asset = tradingPairs.find((asset) => exchangeName == asset.exchange_name);
		return asset.base + '/' + asset.quote;
	}

	function getBase(exchangeName) {
		const asset = tradingPairs.find((asset) => exchangeName == asset.exchange_name);
		return asset.base.toLowerCase();
	}

	function filterTradingPairs() {
		let tmpTradingPairs = tradingPairs;
		console.log(validQuote);
		if (validQuote !== 'all') {
			tmpTradingPairs = tradingPairs.filter((asset) => {
				return asset.quote == validQuote;
			});
		}
		filteredTradingPairs = tmpTradingPairs.filter((asset) => {
			return (
				asset.base.toLowerCase().includes(filterStr.toLowerCase()) ||
				asset.quote.toLowerCase().includes(filterStr.toLowerCase())
			);
		});
	}

	const asset = tradingPairs.find((asset) => market == asset.exchange_name);

	let validQuote = 'all';
	let interval = '60';
	let filteredTradingPairs = tradingPairs;
	let filterStr = '';
</script>

<div class="divider divider-accent text-accent my-0">Market {getMarket(market)}</div>

<div class="navbar">
	<div class="navbar-start">
		<div class="drawer">
			<input id="my-drawer" type="checkbox" class="drawer-toggle" />
			<div class="drawer-content">
				<div class="tooltip tooltip-right tooltip-accent" data-tip="More markets">
					<label for="my-drawer" class="btn btn-accent drawer-button">
						<img
							src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{getBase(
								market
							)}.webp"
							alt={KrakenImg}
						/>
						{getMarket(market)}
					</label>
				</div>
			</div>
			<div class="drawer-side h-full z-50">
				<label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
				<ul class="menu p-4 w-50 min-h-full bg-base-200 text-base-content">
					<input
						type="text"
						placeholder="Search Markets"
						bind:value={filterStr}
						on:input={filterTradingPairs}
						class="input input-bordered input-secondary w-full max-w-xs"
					/>
					<div class="flex items-center gap-1 pt-2 text-secondary">
						<label>
							<input
								type="radio"
								name="radio-1"
								value="all"
								bind:group={validQuote}
								class="radio radio-xs radio-secondary"
							/>
							All
						</label>
						<label>
							<input
								type="radio"
								name="radio-1"
								value="USD"
								bind:group={validQuote}
								class="radio radio-xs radio-secondary"
							/>
							USD
						</label>
						<label>
							<input
								type="radio"
								name="radio-1"
								value="BTC"
								bind:group={validQuote}
								class="radio radio-xs radio-secondary"
							/>
							BTC
						</label>
						<button class="btn btn-secondary btn-xs" on:click={filterTradingPairs}>Filter</button>
					</div>
					<div class="divider divider-accent text-accent">Markets</div>
					<div class="overflow-x-auto h-full text-primary">
						{#each filteredTradingPairs as asset}
							<li>
								<button on:click={viewMarket(asset.exchange_name)}>
									<div class="flex items-center gap-1">
										<div class="avatar">
											<div class="mask mask-squircle w-7 h-7">
												<img
													src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{asset.base.toLowerCase()}.webp"
													alt={KrakenImg}
												/>
											</div>
										</div>
										<div>
											<div class="font-bold">{asset.base}/{asset.quote}</div>
										</div>
									</div>
								</button>
							</li>
						{/each}
					</div>
				</ul>
			</div>
		</div>
	</div>
	<div class="navbar-center">
		<div>
			<label class="text-primary">
				<input
					type="radio"
					name="radio"
					value="1"
					class="radio radio-xs radio-primary"
					bind:group={interval}
				/>
				1 Minute
			</label>
			<label class="text-primary">
				<input
					type="radio"
					name="radio"
					value="5"
					class="radio radio-xs radio-primary"
					bind:group={interval}
				/>
				5 Minute
			</label>
			<label class="text-primary">
				<input
					type="radio"
					name="radio"
					value="30"
					class="radio radio-xs radio-primary"
					bind:group={interval}
				/>
				30 Minute
			</label>
			<label class="text-primary">
				<input
					type="radio"
					name="radio"
					value="60"
					class="radio radio-xs radio-primary"
					bind:group={interval}
				/>
				Hourly
			</label>
			<label class="text-primary">
				<input
					type="radio"
					name="radio"
					value="240"
					class="radio radio-xs radio-primary"
					bind:group={interval}
				/>
				4 Hour
			</label>
			<label class="text-primary">
				<input
					type="radio"
					name="radio"
					value="1440"
					class="radio radio-xs radio-primary"
					bind:group={interval}
				/>
				Daily
			</label>
			<label class="text-primary">
				<input
					type="radio"
					name="radio"
					value="10080"
					class="radio radio-xs radio-primary"
					bind:group={interval}
				/>
				Weekly
			</label>
			<label class="text-primary">
				<input
					type="radio"
					name="radio"
					value="21600"
					class="radio radio-xs radio-primary"
					bind:group={interval}
				/>
				15 Day
			</label>
		</div>
	</div>
	<div class="navbar-end"></div>
</div>
<div class="grid place-items-center text-primary">
	{#key interval}
		<ChartLoader {market} {asset} {interval} />
	{/key}
</div>
