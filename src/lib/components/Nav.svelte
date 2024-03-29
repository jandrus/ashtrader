<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { getUser, logout, unlocked } from '$lib';
	import { minidenticonSvg } from 'https://cdn.jsdelivr.net/npm/minidenticons@4.2.0/minidenticons.min.js';
	import KrakenImg from '$lib/icons/kraken.png';

	let user;
	let isUnlocked;

	async function isOnline() {
		return await invoke('status');
	}

	onMount(() => {
		const interval = setInterval(async () => {
			user = await getUser();
			isUnlocked = await unlocked();
		}, 3000);
		return () => {
			clearInterval(interval);
		};
	});
</script>

<div class="navbar">
	<div class="navbar-start">
		{#if isOnline()}
			<div class="tooltip tooltip-right mt-2 ml-2" data-tip="Kraken online">
				<div class="avatar online">
					<a href="/">
						<div class="mask mask-squircle w-7 h-7">
							<img
								src={KrakenImg}
								alt="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/64/doge.webp"
							/>
						</div>
					</a>
				</div>
			</div>
		{:else}
			<div class="tooltip tooltip-right mt-2 ml-2" data-tip="Kraken offline">
				<div class="avatar offline">
					<div class="mask mask-squircle w-7 h-7">
						<img
							src={KrakenImg}
							alt="https://lcw.nyc3.cdn.digitaloceanspaces.com/production/currencies/64/doge.webp"
						/>
					</div>
				</div>
			</div>
		{/if}
	</div>
	<div class="navbar-center space-x-2">
		<a class="btn btn-ghost text-xl text-primary" href="/">ASH Trader</a>
	</div>
	<div class="navbar-end">
		{#if user}
			{#if user.kraken_auth}
				{#if isUnlocked}
					<div
						class="tooltip tooltip-success tooltip-left mt-2 ml-2"
						data-tip="Account unlocked. Trading available"
					>
						<div class="mask mask-squircle w-9 h-9">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
								class="fill-success w-8 h-8"
							>
								<path
									d="M18 1.5c2.9 0 5.25 2.35 5.25 5.25v3.75a.75.75 0 0 1-1.5 0V6.75a3.75 3.75 0 1 0-7.5 0v3a3 3 0 0 1 3 3v6.75a3 3 0 0 1-3 3H3.75a3 3 0 0 1-3-3v-6.75a3 3 0 0 1 3-3h9v-3c0-2.9 2.35-5.25 5.25-5.25Z"
								/>
							</svg>
						</div>
					</div>
				{:else}
					<div
						class="tooltip tooltip-error tooltip-left mt-2 ml-2"
						data-tip="Account locked. Unlock with password."
					>
						<div class="mask mask-squircle w-9 h-9">
							<a href="/unlock">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									class="fill-error w-8 h-8"
								>
									<path
										fill-rule="evenodd"
										d="M12 1.5a5.25 5.25 0 0 0-5.25 5.25v3a3 3 0 0 0-3 3v6.75a3 3 0 0 0 3 3h10.5a3 3 0 0 0 3-3v-6.75a3 3 0 0 0-3-3v-3c0-2.9-2.35-5.25-5.25-5.25Zm3.75 8.25v-3a3.75 3.75 0 1 0-7.5 0v3h7.5Z"
										clip-rule="evenodd"
									/>
								</svg>
							</a>
						</div>
					</div>
				{/if}
			{:else}
				<div
					class="tooltip tooltip-error tooltip-left mt-2 ml-2"
					data-tip="Add Kraken API keys to enable trading."
				>
					<div class="mask mask-squircle w-9 h-9">
						<a href="/addKraken">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
								class="fill-error w-8 h-8"
							>
								<path
									fill-rule="evenodd"
									d="M15.75 1.5a6.75 6.75 0 0 0-6.651 7.906c.067.39-.032.717-.221.906l-6.5 6.499a3 3 0 0 0-.878 2.121v2.818c0 .414.336.75.75.75H6a.75.75 0 0 0 .75-.75v-1.5h1.5A.75.75 0 0 0 9 19.5V18h1.5a.75.75 0 0 0 .53-.22l2.658-2.658c.19-.189.517-.288.906-.22A6.75 6.75 0 1 0 15.75 1.5Zm0 3a.75.75 0 0 0 0 1.5A2.25 2.25 0 0 1 18 8.25a.75.75 0 0 0 1.5 0 3.75 3.75 0 0 0-3.75-3.75Z"
									clip-rule="evenodd"
								/>
							</svg>
						</a>
					</div>
				</div>
			{/if}
		{/if}
		<div class="dropdown dropdown-end">
			<div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
				<!-- IDENTICON -->
				<div class="flex max-w-xs items-center rounded-full bg-gray-800 text-sm">
					<div class="h-10 w-10 rounded-full border border-primary">
						{#if user}
							<minidenticon-svg username={user.username}></minidenticon-svg>
						{/if}
					</div>
				</div>
				<!-- /IDENTICON -->
			</div>
			<ul
				tabindex="-1"
				class="menu menu-sm dropdown-content mt-1 z-[5] p-1 shadow bg-base-100 rounded-box w-52 text-primary"
			>
				<li>
					<a href="/" class="justify-between">
						Home
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-5 w-5"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
							/></svg
						>
					</a>
				</li>
				<li>
					<a href="/market" class="justify-between">
						Market
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 24 24"
							fill="currentColor"
							class="w-5 h-5"
						>
							<path
								fill-rule="evenodd"
								d="M9.315 7.584C12.195 3.883 16.695 1.5 21.75 1.5a.75.75 0 0 1 .75.75c0 5.056-2.383 9.555-6.084 12.436A6.75 6.75 0 0 1 9.75 22.5a.75.75 0 0 1-.75-.75v-4.131A15.838 15.838 0 0 1 6.382 15H2.25a.75.75 0 0 1-.75-.75 6.75 6.75 0 0 1 7.815-6.666ZM15 6.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5Z"
								clip-rule="evenodd"
							/>
							<path
								d="M5.26 17.242a.75.75 0 1 0-.897-1.203 5.243 5.243 0 0 0-2.05 5.022.75.75 0 0 0 .625.627 5.243 5.243 0 0 0 5.022-2.051.75.75 0 1 0-1.202-.897 3.744 3.744 0 0 1-3.008 1.51c0-1.23.592-2.323 1.51-3.008Z"
							/>
						</svg>
					</a>
				</li>
				{#if user}
					<li>
						<a href="/settings" class="justify-between">Settings</a>
					</li>
					<li>
						<a href="/portfolio" class="justify-between">Portfolio</a>
					</li>
					{#if !user.kraken_auth}
						<li>
							<a href="/addKraken" class="justify-between">Add Kraken API Keys</a>
						</li>
					{/if}
					{#if !user.ash_auth}
						<li>
							<a href="/addAsh" class="justify-between">Add ASH API Keys</a>
						</li>
					{/if}
					<li>
						<button data-sveltekit-reload on:click={logout} class="justify-between">
							Logout
						</button>
					</li>
				{:else}
					<li>
						<a href="/login" class="justify-between">Login</a>
					</li>
					<li>
						<a href="/register" class="justify-between">Register</a>
					</li>
				{/if}
			</ul>
		</div>
	</div>
</div>
