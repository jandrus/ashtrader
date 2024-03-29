<script>
	import { onMount } from 'svelte';
	import { getTradingPairs, viewMarket } from '$lib';
	import Loading from '$lib/components/Loading.svelte';
	import KrakenImg from '$lib/icons/kraken.png';

	let tradingPairs = [];

	let sortBy = { col: 'base', ascending: true };

	let allQuotes = [];
	let validQuotes = [];

	function getQuotes() {
		if (tradingPairs) {
			let quotes = [];
			for (var i = 0; i < tradingPairs.length; i++) {
				if (!quotes.includes(tradingPairs[i].quote)) {
					quotes.push(tradingPairs[i].quote);
				}
			}
			return quotes.sort();
		}
	}

	function toggleAll() {
		if (allSelected) {
			validQuotes = [];
		} else {
			validQuotes = allQuotes;
		}
	}

	$: sort = (column) => {
		if (sortBy.col == column) {
			sortBy.ascending = !sortBy.ascending;
		} else {
			sortBy.col = column;
			sortBy.ascending = true;
		}
		let sortModifier = sortBy.ascending ? 1 : -1;
		let sort = (a, b) =>
			a[column] < b[column] ? -1 * sortModifier : a[column] > b[column] ? 1 * sortModifier : 0;
		tradingPairs = tradingPairs.sort(sort);
	};

	$: allSelected = validQuotes.length === allQuotes.length;

	onMount(async () => {
		tradingPairs = await getTradingPairs();
		allQuotes = getQuotes();
		validQuotes = getQuotes();
		const interval = setInterval(async () => {
			tradingPairs = await getTradingPairs();
		}, 300000);
		return () => {
			clearInterval(interval);
		};
	});
</script>

<div class="divider divider-accent text-accent my-0">Trading Pairs</div>
{#if tradingPairs}
	<div class="flex">
		{#each allQuotes as quote}
			<div class="form-control">
				<label class="label cursor-pointer">
					<span class="label-text text-primary">{quote.toUpperCase()} Pairs</span>
					<input
						type="checkbox"
						checked="checked"
						value={quote.toUpperCase()}
						bind:group={validQuotes}
						class="checkbox checkbox-primary checkbox-xs mx-1"
					/>
				</label>
			</div>
		{/each}
		<div class="form-control">
			<label class="label cursor-pointer">
				<span class="label-text text-secondary">All</span>
				<input
					on:change={toggleAll}
					checked={allSelected}
					type="checkbox"
					class="toggle toggle-xs toggle-secondary mx-1"
				/>
			</label>
		</div>
	</div>
	<div class="overflow-x-auto" style="height: 41rem;">
		<table class="table table-pin-rows bg-neutral">
			<thead class="text-primary">
				<tr>
					<th>
						<button on:click={sort('base')}>
							<div class="flex items-center gap-1">
								Pair
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-4 h-4"
								>
									<path
										fill-rule="evenodd"
										d="M6.97 2.47a.75.75 0 0 1 1.06 0l4.5 4.5a.75.75 0 0 1-1.06 1.06L8.25 4.81V16.5a.75.75 0 0 1-1.5 0V4.81L3.53 8.03a.75.75 0 0 1-1.06-1.06l4.5-4.5Zm9.53 4.28a.75.75 0 0 1 .75.75v11.69l3.22-3.22a.75.75 0 1 1 1.06 1.06l-4.5 4.5a.75.75 0 0 1-1.06 0l-4.5-4.5a.75.75 0 1 1 1.06-1.06l3.22 3.22V7.5a.75.75 0 0 1 .75-.75Z"
										clip-rule="evenodd"
									/>
								</svg>
							</div>
						</button>
					</th>
					<th> Price </th>
					<th> High (24hr) </th>
					<th>
						<button on:click={sort('delta')}>
							<div class="flex items-center gap-1">
								Change (24hr)
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-4 h-4"
								>
									<path
										fill-rule="evenodd"
										d="M6.97 2.47a.75.75 0 0 1 1.06 0l4.5 4.5a.75.75 0 0 1-1.06 1.06L8.25 4.81V16.5a.75.75 0 0 1-1.5 0V4.81L3.53 8.03a.75.75 0 0 1-1.06-1.06l4.5-4.5Zm9.53 4.28a.75.75 0 0 1 .75.75v11.69l3.22-3.22a.75.75 0 1 1 1.06 1.06l-4.5 4.5a.75.75 0 0 1-1.06 0l-4.5-4.5a.75.75 0 1 1 1.06-1.06l3.22 3.22V7.5a.75.75 0 0 1 .75-.75Z"
										clip-rule="evenodd"
									/>
								</svg>
							</div>
						</button>
					</th>
				</tr>
			</thead>
			<tbody class="text-accent">
				{#each tradingPairs as asset}
					{#if validQuotes.includes(asset.quote)}
						<tr>
							<!-- PAIR -->
							<td>
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
											<div class="font-bold underline">{asset.base}/{asset.quote}</div>
										</div>
									</div>
								</button>
							</td>
							<!-- PRICE -->
							<td>
								{#if asset.quote == 'USD'}
									${asset.price.toFixed(asset.quote_decimals)}
								{:else if asset.quote == 'BTC'}
									&#8383{asset.price.toFixed(asset.quote_decimals)}
								{:else}
									{asset.price.toFixed(asset.quote_decimals)}
								{/if}
							</td>
							<!-- HIGH -->
							<td>
								{#if asset.quote == 'USD'}
									${asset.high.toFixed(asset.quote_decimals)}
								{:else if asset.quote == 'BTC'}
									&#8383{asset.high.toFixed(asset.quote_decimals)}
								{:else}
									{asset.high.toFixed(asset.quote_decimals)}
								{/if}
							</td>
							<!-- DELTA -->
							{#if asset.delta > 0}
								<td>
									{(asset.delta * 100).toFixed(2)}%
								</td>
							{:else}
								<td class="text-error">
									{(asset.delta * 100).toFixed(2)}%
								</td>
							{/if}
						</tr>
					{/if}
				{/each}
			</tbody>
		</table>
	</div>
{:else}
	<Loading />
{/if}
