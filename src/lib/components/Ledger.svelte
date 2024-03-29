<script>
	import { onMount } from 'svelte';
	import { DateInput } from 'date-picker-svelte';
	import KrakenImg from '$lib/icons/kraken.png';

	export let ledger;

	let filteredLedger = [];
	let min = new Date('2008-11-01');
	let startDate = null;
	let endDate = null;
	let assets = [];
	let selectedAsset = 'all';
	let selectedType = 'all';

	function toDate(ts) {
		let d = new Date(ts * 1000);
		return d.toDateString();
	}

	function filterLedger() {
		if (selectedAsset == 'all' && selectedType == 'all') {
			filteredLedger = ledger;
		} else if (selectedAsset == 'all') {
			filteredLedger = [];
			ledger.forEach((t) => {
				if (t.type_field == selectedType) {
					filteredLedger.push(t);
				}
			});
		} else if (selectedType == 'all') {
			filteredLedger = [];
			ledger.forEach((t) => {
				if (t.asset == selectedAsset) {
					filteredLedger.push(t);
				}
			});
		} else {
			filteredLedger = [];
			ledger.forEach((t) => {
				if (t.asset == selectedAsset && t.type_field == selectedType) {
					filteredLedger.push(t);
				}
			});
		}
	}

	function filterDates() {
		if (startDate && endDate && ledger) {
			filterLedger();
			let tmpLedger = filteredLedger;
			filteredLedger = [];
			tmpLedger.forEach((t) => {
				if (t.time * 1000 >= startDate.getTime() && t.time * 1000 <= endDate.getTime()) {
					filteredLedger.push(t);
				}
			});
		}
	}

	let sortBy = { col: 'time', ascending: true };

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
		if (startDate && endDate) {
			ledger = ledger.sort(sort);
		} else {
			filteredLedger = filteredLedger.sort(sort);
		}
	};

	onMount(() => {
		ledger.forEach((t) => {
			if (!assets.includes(t.asset)) {
				assets.push(t.asset);
			}
		});
		assets = assets.sort((a, b) => {
			if (a < b) {
				return -1;
			}
			if (a > b) {
				return 1;
			}
			return 0;
		});
		filteredLedger = ledger;
		const interval = setInterval(() => {
			filterDates();
		}, 1000);
		return () => {
			clearInterval(interval);
		};
	});
</script>

<div class="hero min-h my-0">
	<div class="navbar text-primary">
		<div class="navbar-start"></div>
		<div class="navbar-end">
			<div class="date-filter text-xs mx-2">
				<DateInput
					bind:value={startDate}
					closeOnSelection="true"
					{min}
					format="yyyy-MM-dd"
					placeholder="Start date"
				/>
			</div>
			<h1 class="text-sm">to</h1>
			<div class="date-filter text-xs mx-2">
				<DateInput
					bind:value={endDate}
					closeOnSelection="true"
					{min}
					format="yyyy-MM-dd"
					placeholder="End date"
				/>
			</div>
			<div class="tooltip tooltip-left tooltip-primary" data-tip="Past 7 days">
				<button
					class="mx-1 btn btn-sm btn-outline btn-primary"
					on:click={() => {
						endDate = new Date();
						startDate = new Date();
						startDate.setDate(startDate.getDate() - 7);
					}}
				>
					7 Days
				</button>
			</div>
			<div class="tooltip tooltip-left tooltip-primary" data-tip="Past 30 days">
				<button
					class="mx-1 btn btn-sm btn-outline btn-primary"
					on:click={() => {
						endDate = new Date();
						startDate = new Date();
						startDate.setDate(startDate.getDate() - 30);
					}}
				>
					30 Days
				</button>
			</div>
			<div class="tooltip tooltip-left tooltip-primary" data-tip="Past 60 days">
				<button
					class="mx-1 btn btn-sm btn-outline btn-primary"
					on:click={() => {
						endDate = new Date();
						startDate = new Date();
						startDate.setDate(startDate.getDate() - 60);
					}}
				>
					60 Days
				</button>
			</div>
			<div class="tooltip tooltip-left tooltip-primary" data-tip="All Transactions">
				<button
					class="mx-1 btn btn-sm btn-outline btn-primary"
					on:click={() => {
						startDate = null;
						endDate = null;
						filterLedger();
						/* startDate = min;
							 endDate = new Date(); */
					}}
				>
					Clear Dates
				</button>
			</div>
		</div>
	</div>
</div>
<div class="overflow-x-auto" style="height: 34rem;">
	{#if ledger}
		<table class="table table-sm table-pin-rows bg-neutral">
			<thead class="text-primary">
				<tr>
					<th>
						<select
							class="select select-xs select-primary"
							bind:value={selectedAsset}
							on:change={filterLedger}
						>
							<option selected value="all">All</option>
							{#each assets as asset}
								<option value={asset}>{asset}</option>
							{/each}
						</select>
					</th>
					<th>
						<select
							class="select select-xs select-primary"
							bind:value={selectedType}
							on:change={filterLedger}
						>
							<option selected value="all">All</option>
							<option value="buy">Buy</option>
							<option value="deposit">Deposit</option>
							<option value="sell">Sell</option>
							<option value="withdrawal">Withdrawal</option>
						</select>
					</th>
					<th> Amount </th>
					<th> Fee </th>
					<th> Account Balance </th>
					<th>
						<button on:click={sort('time')}>
							<div class="flex items-center gap-1">
								Date
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
					<th> Tx ID </th>
				</tr>
			</thead>
			<tbody class="text-accent">
				{#each filteredLedger as transaction}
					{#if transaction.type_field == 'deposit'}
						<!-- DEPOSIT -->
						<tr class="hover text-success">
							<!-- Asset -->
							<td>
								<div class="flex items-center gap-1">
									<div class="avatar">
										<div class="mask mask-squircle w-6 h-6">
											<img
												src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{transaction.asset.toLowerCase()}.webp"
												alt={KrakenImg}
											/>
										</div>
									</div>
									<div>
										<div class="font-bold">{transaction.asset}</div>
									</div>
								</div>
							</td>
							<!-- TYPE -->
							<td> Deposit </td>
							<!-- Amount -->
							<td> {transaction.amount} </td>
							<!-- FEE -->
							<td> {transaction.fee} </td>
							<!-- BALANCE -->
							<td> {transaction.balance} </td>
							<!-- DATE -->
							<td> {toDate(transaction.time)} </td>
							<!-- TXID -->
							<td class="tooltip tooltip-left tooltip-success" data-tip="refid: {transaction.txid}">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-6 h-6"
								>
									<path
										fill-rule="evenodd"
										d="M12 3.75a6.715 6.715 0 0 0-3.722 1.118.75.75 0 1 1-.828-1.25 8.25 8.25 0 0 1 12.8 6.883c0 3.014-.574 5.897-1.62 8.543a.75.75 0 0 1-1.395-.551A21.69 21.69 0 0 0 18.75 10.5 6.75 6.75 0 0 0 12 3.75ZM6.157 5.739a.75.75 0 0 1 .21 1.04A6.715 6.715 0 0 0 5.25 10.5c0 1.613-.463 3.12-1.265 4.393a.75.75 0 0 1-1.27-.8A6.715 6.715 0 0 0 3.75 10.5c0-1.68.503-3.246 1.367-4.55a.75.75 0 0 1 1.04-.211ZM12 7.5a3 3 0 0 0-3 3c0 3.1-1.176 5.927-3.105 8.056a.75.75 0 1 1-1.112-1.008A10.459 10.459 0 0 0 7.5 10.5a4.5 4.5 0 1 1 9 0c0 .547-.022 1.09-.067 1.626a.75.75 0 0 1-1.495-.123c.041-.495.062-.996.062-1.503a3 3 0 0 0-3-3Zm0 2.25a.75.75 0 0 1 .75.75c0 3.908-1.424 7.485-3.781 10.238a.75.75 0 0 1-1.14-.975A14.19 14.19 0 0 0 11.25 10.5a.75.75 0 0 1 .75-.75Zm3.239 5.183a.75.75 0 0 1 .515.927 19.417 19.417 0 0 1-2.585 5.544.75.75 0 0 1-1.243-.84 17.915 17.915 0 0 0 2.386-5.116.75.75 0 0 1 .927-.515Z"
										clip-rule="evenodd"
									/>
								</svg>
							</td>
						</tr>
					{:else if transaction.type_field == 'withdrawal'}
						<!-- WITHDRAWAL -->
						<tr class="hover text-error">
							<!-- Asset -->
							<td>
								<div class="flex items-center gap-1">
									<div class="avatar">
										<div class="mask mask-squircle w-6 h-6">
											<img
												src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{transaction.asset.toLowerCase()}.webp"
												alt={KrakenImg}
											/>
										</div>
									</div>
									<div>
										<div class="font-bold">{transaction.asset}</div>
									</div>
								</div>
							</td>
							<!-- TYPE -->
							<td> Withdrawal </td>
							<!-- Amount -->
							<td> {transaction.amount} </td>
							<!-- FEE -->
							<td> {transaction.fee} </td>
							<!-- BALANCE -->
							<td> {transaction.balance} </td>
							<!-- DATE -->
							<td> {toDate(transaction.time)} </td>
							<!-- TXID -->
							<td class="tooltip tooltip-left tooltip-error" data-tip="txid: {transaction.txid}">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-6 h-6"
								>
									<path
										fill-rule="evenodd"
										d="M12 3.75a6.715 6.715 0 0 0-3.722 1.118.75.75 0 1 1-.828-1.25 8.25 8.25 0 0 1 12.8 6.883c0 3.014-.574 5.897-1.62 8.543a.75.75 0 0 1-1.395-.551A21.69 21.69 0 0 0 18.75 10.5 6.75 6.75 0 0 0 12 3.75ZM6.157 5.739a.75.75 0 0 1 .21 1.04A6.715 6.715 0 0 0 5.25 10.5c0 1.613-.463 3.12-1.265 4.393a.75.75 0 0 1-1.27-.8A6.715 6.715 0 0 0 3.75 10.5c0-1.68.503-3.246 1.367-4.55a.75.75 0 0 1 1.04-.211ZM12 7.5a3 3 0 0 0-3 3c0 3.1-1.176 5.927-3.105 8.056a.75.75 0 1 1-1.112-1.008A10.459 10.459 0 0 0 7.5 10.5a4.5 4.5 0 1 1 9 0c0 .547-.022 1.09-.067 1.626a.75.75 0 0 1-1.495-.123c.041-.495.062-.996.062-1.503a3 3 0 0 0-3-3Zm0 2.25a.75.75 0 0 1 .75.75c0 3.908-1.424 7.485-3.781 10.238a.75.75 0 0 1-1.14-.975A14.19 14.19 0 0 0 11.25 10.5a.75.75 0 0 1 .75-.75Zm3.239 5.183a.75.75 0 0 1 .515.927 19.417 19.417 0 0 1-2.585 5.544.75.75 0 0 1-1.243-.84 17.915 17.915 0 0 0 2.386-5.116.75.75 0 0 1 .927-.515Z"
										clip-rule="evenodd"
									/>
								</svg>
							</td>
						</tr>
					{:else if transaction.type_field == 'buy'}
						<!-- BUY -->
						<tr class="hover text-info">
							<!-- Asset -->
							<td>
								<div class="flex items-center gap-1">
									<div class="avatar">
										<div class="mask mask-squircle w-6 h-6">
											<img
												src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{transaction.asset.toLowerCase()}.webp"
												alt={KrakenImg}
											/>
										</div>
									</div>
									<div>
										<div class="font-bold">{transaction.asset}</div>
									</div>
								</div>
							</td>
							<!-- TYPE -->
							<td> Buy </td>
							<!-- Amount -->
							<td> {transaction.amount} </td>
							<!-- FEE -->
							<td> {transaction.fee} </td>
							<!-- BALANCE -->
							<td> {transaction.balance} </td>
							<!-- DATE -->
							<td> {toDate(transaction.time)} </td>
							<!-- TXID -->
							<td class="tooltip tooltip-left tooltip-info" data-tip="refid: {transaction.refid}">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-6 h-6"
								>
									<path
										fill-rule="evenodd"
										d="M12 3.75a6.715 6.715 0 0 0-3.722 1.118.75.75 0 1 1-.828-1.25 8.25 8.25 0 0 1 12.8 6.883c0 3.014-.574 5.897-1.62 8.543a.75.75 0 0 1-1.395-.551A21.69 21.69 0 0 0 18.75 10.5 6.75 6.75 0 0 0 12 3.75ZM6.157 5.739a.75.75 0 0 1 .21 1.04A6.715 6.715 0 0 0 5.25 10.5c0 1.613-.463 3.12-1.265 4.393a.75.75 0 0 1-1.27-.8A6.715 6.715 0 0 0 3.75 10.5c0-1.68.503-3.246 1.367-4.55a.75.75 0 0 1 1.04-.211ZM12 7.5a3 3 0 0 0-3 3c0 3.1-1.176 5.927-3.105 8.056a.75.75 0 1 1-1.112-1.008A10.459 10.459 0 0 0 7.5 10.5a4.5 4.5 0 1 1 9 0c0 .547-.022 1.09-.067 1.626a.75.75 0 0 1-1.495-.123c.041-.495.062-.996.062-1.503a3 3 0 0 0-3-3Zm0 2.25a.75.75 0 0 1 .75.75c0 3.908-1.424 7.485-3.781 10.238a.75.75 0 0 1-1.14-.975A14.19 14.19 0 0 0 11.25 10.5a.75.75 0 0 1 .75-.75Zm3.239 5.183a.75.75 0 0 1 .515.927 19.417 19.417 0 0 1-2.585 5.544.75.75 0 0 1-1.243-.84 17.915 17.915 0 0 0 2.386-5.116.75.75 0 0 1 .927-.515Z"
										clip-rule="evenodd"
									/>
								</svg>
							</td>
						</tr>
					{:else if transaction.type_field == 'sell'}
						<!-- SELL -->
						<tr class="hover text-warning">
							<!-- Asset -->
							<td>
								<div class="flex items-center gap-1">
									<div class="avatar">
										<div class="mask mask-squircle w-6 h-6">
											<img
												src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/{transaction.asset.toLowerCase()}.webp"
												alt={KrakenImg}
											/>
										</div>
									</div>
									<div>
										<div class="font-bold">{transaction.asset}</div>
									</div>
								</div>
							</td>
							<!-- TYPE -->
							<td> Sell </td>
							<!-- Amount -->
							<td> {transaction.amount} </td>
							<!-- FEE -->
							<td> {transaction.fee} </td>
							<!-- BALANCE -->
							<td> {transaction.balance} </td>
							<!-- DATE -->
							<td> {toDate(transaction.time)} </td>
							<!-- TXID -->
							<td
								class="tooltip tooltip-left tooltip-warning"
								data-tip="refid: {transaction.refid}"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-6 h-6"
								>
									<path
										fill-rule="evenodd"
										d="M12 3.75a6.715 6.715 0 0 0-3.722 1.118.75.75 0 1 1-.828-1.25 8.25 8.25 0 0 1 12.8 6.883c0 3.014-.574 5.897-1.62 8.543a.75.75 0 0 1-1.395-.551A21.69 21.69 0 0 0 18.75 10.5 6.75 6.75 0 0 0 12 3.75ZM6.157 5.739a.75.75 0 0 1 .21 1.04A6.715 6.715 0 0 0 5.25 10.5c0 1.613-.463 3.12-1.265 4.393a.75.75 0 0 1-1.27-.8A6.715 6.715 0 0 0 3.75 10.5c0-1.68.503-3.246 1.367-4.55a.75.75 0 0 1 1.04-.211ZM12 7.5a3 3 0 0 0-3 3c0 3.1-1.176 5.927-3.105 8.056a.75.75 0 1 1-1.112-1.008A10.459 10.459 0 0 0 7.5 10.5a4.5 4.5 0 1 1 9 0c0 .547-.022 1.09-.067 1.626a.75.75 0 0 1-1.495-.123c.041-.495.062-.996.062-1.503a3 3 0 0 0-3-3Zm0 2.25a.75.75 0 0 1 .75.75c0 3.908-1.424 7.485-3.781 10.238a.75.75 0 0 1-1.14-.975A14.19 14.19 0 0 0 11.25 10.5a.75.75 0 0 1 .75-.75Zm3.239 5.183a.75.75 0 0 1 .515.927 19.417 19.417 0 0 1-2.585 5.544.75.75 0 0 1-1.243-.84 17.915 17.915 0 0 0 2.386-5.116.75.75 0 0 1 .927-.515Z"
										clip-rule="evenodd"
									/>
								</svg>
							</td>
						</tr>
					{/if}
				{/each}
			</tbody>
		</table>
	{/if}
</div>

<style>
	:root {
		--date-picker-background: #1b1e27;
		--date-picker-foreground: primary;
	}
</style>
