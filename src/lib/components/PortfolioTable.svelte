<script>
	import KrakenImg from '$lib/icons/kraken.png';

	export let portfolio;
	export let usdValue;

	let sortBy = { col: 'name', ascending: true };

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
		portfolio = portfolio.sort(sort);
	};
</script>

<div class="overflow-x-auto" style="height: 38rem;">
	{#if portfolio}
		<table class="table table-pin-rows bg-neutral">
			<thead class="text-primary">
				<tr>
					<th>
						<button on:click={sort('name')}>
							<div class="flex items-center gap-1">
								Asset
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
					<th>
						{#if usdValue}
							<button on:click={sort('priceUsd')}>
								<div class="flex items-center gap-1">
									Price
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
						{:else}
							<button on:click={sort('priceBtc')}>
								<div class="flex items-center gap-1">
									Price
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
						{/if}
					</th>
					<th>
						<button on:click={sort('balance')}>
							<div class="flex items-center gap-1">
								Balance
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
					<th>
						{#if usdValue}
							<button on:click={sort('valueUsd')}>
								<div class="flex items-center gap-1">
									Value
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
						{:else}
							<button on:click={sort('valueBtc')}>
								<div class="flex items-center gap-1">
									Value
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
						{/if}
					</th>
				</tr>
			</thead>
			<tbody class="text-accent">
				{#each portfolio as asset}
					<tr class="hover">
						<!-- PAIR -->
						<td>
							<div class="flex items-center gap-1">
								<div class="avatar">
									<div class="mask mask-squircle w-7 h-7">
										<img
											src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{asset.name.toLowerCase()}.webp"
											alt={KrakenImg}
										/>
									</div>
								</div>
								<div>
									<div class="font-bold">{asset.name}</div>
								</div>
							</div>
						</td>
						<!-- PRICE -->
						<td>
							{#if usdValue}
								${asset.priceUsd}
							{:else}
								&#8383{asset.priceBtc}
							{/if}
						</td>
						<!-- BALANCE -->
						<td>
							{asset.balance}
						</td>
						<!-- VALUE -->
						<td>
							{#if usdValue}
								${asset.valueUsd.toFixed(2)}
							{:else}
								&#8383{asset.valueBtc.toFixed(8)}
							{/if}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>
