<script>
	import { onMount } from 'svelte';
	import { getLedger, getPortfolio, getUser } from '$lib';
	import PortfolioTable from '$lib/components/PortfolioTable.svelte';
	import Ledger from '$lib/components/Ledger.svelte';
	import KrakenImg from '$lib/icons/kraken.png';

	let user = '';
	let portfolio = [];
	let ledger = [];
	let usdValue = false;
	let section = 'balances';
	let totalUsd = 0;
	let totalBtc = 0;

	function getAccountValue() {
		totalUsd = 0;
		totalBtc = 0;
		if (portfolio) {
			portfolio.forEach((a) => {
				totalUsd += parseFloat(a.valueUsd);
				totalBtc += parseFloat(a.valueBtc);
			});
		}
		totalUsd = totalUsd.toFixed(2).replace(/\B(?<!\.\d*)(?=(\d{3})+(?!\d))/g, ',');
		totalBtc = totalBtc.toFixed(8);
	}

	onMount(async () => {
		user = await getUser();
		portfolio = await getPortfolio();
		ledger = await getLedger();
		getAccountValue();
		const interval = setInterval(async () => {
			portfolio = await getPortfolio();
			getAccountValue();
		}, 6000000);
		return () => {
			clearInterval(interval);
		};
	});
</script>

{#if section == 'balances'}
	<div class="divider divider-accent text-accent my-0">Portfolio Balances</div>
{:else}
	<div class="divider divider-accent text-accent my-0">Portfolio Transaction Ledger</div>
{/if}
{#if user}
	<div class="hero min-h">
		<div class="navbar text-primary">
			<div class="navbar-start">
				<label class="swap swap-rotate">
					<input type="checkbox" bind:checked={usdValue} />
					<div class="skeleton w-12 h-12 rounded-full"></div>
					<div class="swap-on rounded-full w-8 h-8 mt-2 ml-2">
						<img
							src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/usd.webp"
							alt={KrakenImg}
						/>
					</div>
					<div
						class="swap-off tooltip tooltip-accent tooltip-right rounded-full w-8 h-8 mt-2 ml-2 z-10"
						data-tip="Click to show in USD"
					>
						<img
							src="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/32/btc.webp"
							alt={KrakenImg}
						/>
					</div>
				</label>
				<h1 class="text-2xl ml-3">
					Account Value:
					{#if portfolio}
						{#if usdValue}
							${totalUsd}
						{:else}
							&#8383{totalBtc}
						{/if}
					{:else}
						<span class="loading loading-bars loading-md"></span>
					{/if}
				</h1>
			</div>
			<div class="navbar-end">
				<ul class="menu menu-horizontal rounded-box">
					<li>
						{#if section == 'balances'}
							<button class="btn-active tooltip" data-tip="Balances">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-6 h-6"
								>
									<path
										d="M11.584 2.376a.75.75 0 0 1 .832 0l9 6a.75.75 0 1 1-.832 1.248L12 3.901 3.416 9.624a.75.75 0 0 1-.832-1.248l9-6Z"
									/>
									<path
										fill-rule="evenodd"
										d="M20.25 10.332v9.918H21a.75.75 0 0 1 0 1.5H3a.75.75 0 0 1 0-1.5h.75v-9.918a.75.75 0 0 1 .634-.74A49.109 49.109 0 0 1 12 9c2.59 0 5.134.202 7.616.592a.75.75 0 0 1 .634.74Zm-7.5 2.418a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Zm3-.75a.75.75 0 0 1 .75.75v6.75a.75.75 0 0 1-1.5 0v-6.75a.75.75 0 0 1 .75-.75ZM9 12.75a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Z"
										clip-rule="evenodd"
									/>
									<path d="M12 7.875a1.125 1.125 0 1 0 0-2.25 1.125 1.125 0 0 0 0 2.25Z" />
								</svg>
							</button>
						{:else}
							<button class="tooltip" data-tip="Balances" on:click={() => (section = 'balances')}>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-6 h-6"
								>
									<path
										d="M11.584 2.376a.75.75 0 0 1 .832 0l9 6a.75.75 0 1 1-.832 1.248L12 3.901 3.416 9.624a.75.75 0 0 1-.832-1.248l9-6Z"
									/>
									<path
										fill-rule="evenodd"
										d="M20.25 10.332v9.918H21a.75.75 0 0 1 0 1.5H3a.75.75 0 0 1 0-1.5h.75v-9.918a.75.75 0 0 1 .634-.74A49.109 49.109 0 0 1 12 9c2.59 0 5.134.202 7.616.592a.75.75 0 0 1 .634.74Zm-7.5 2.418a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Zm3-.75a.75.75 0 0 1 .75.75v6.75a.75.75 0 0 1-1.5 0v-6.75a.75.75 0 0 1 .75-.75ZM9 12.75a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Z"
										clip-rule="evenodd"
									/>
									<path d="M12 7.875a1.125 1.125 0 1 0 0-2.25 1.125 1.125 0 0 0 0 2.25Z" />
								</svg>
							</button>
						{/if}
					</li>
					<li>
						{#if section == 'ledger'}
							<button class="btn-active tooltip" data-tip="Ledger">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-6 h-6"
								>
									<path d="M4.5 3.75a3 3 0 0 0-3 3v.75h21v-.75a3 3 0 0 0-3-3h-15Z" />
									<path
										fill-rule="evenodd"
										d="M22.5 9.75h-21v7.5a3 3 0 0 0 3 3h15a3 3 0 0 0 3-3v-7.5Zm-18 3.75a.75.75 0 0 1 .75-.75h6a.75.75 0 0 1 0 1.5h-6a.75.75 0 0 1-.75-.75Zm.75 2.25a.75.75 0 0 0 0 1.5h3a.75.75 0 0 0 0-1.5h-3Z"
										clip-rule="evenodd"
									/>
								</svg>
							</button>
						{:else}
							<button class="tooltip" data-tip="Ledger" on:click={() => (section = 'ledger')}>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="currentColor"
									class="w-6 h-6"
								>
									<path d="M4.5 3.75a3 3 0 0 0-3 3v.75h21v-.75a3 3 0 0 0-3-3h-15Z" />
									<path
										fill-rule="evenodd"
										d="M22.5 9.75h-21v7.5a3 3 0 0 0 3 3h15a3 3 0 0 0 3-3v-7.5Zm-18 3.75a.75.75 0 0 1 .75-.75h6a.75.75 0 0 1 0 1.5h-6a.75.75 0 0 1-.75-.75Zm.75 2.25a.75.75 0 0 0 0 1.5h3a.75.75 0 0 0 0-1.5h-3Z"
										clip-rule="evenodd"
									/>
								</svg>
							</button>
						{/if}
					</li>
				</ul>
				<!-- <h1 class="text-4xl font-bold">
					 {user.username.charAt(0).toUpperCase() + user.username.slice(1)}
					 </h1> -->
			</div>
		</div>
	</div>
{/if}
{#if section == 'balances'}
	<PortfolioTable {portfolio} {usdValue} />
{:else if section == 'ledger'}
	<Ledger {ledger} />
{/if}
