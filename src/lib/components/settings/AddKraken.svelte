<script>
	import AddKrakenLoader from '$lib/loaders/AddKrakenLoader.svelte';

	export let err;

	let krakenApi = '';
	let krakenSk = '';
	let password = '';
	let show = false;
	let errMsg = err ? err : '';
	let submit = false;

	function addKeys() {
		errMsg = '';
		if (krakenApi.length != 56) {
			errMsg = 'Invalid API Key';
			krakenApi = '';
			return;
		}
		if (krakenSk.length != 88) {
			errMsg = 'Invalid Private Key';
			krakenSk = '';
			return;
		}
		submit = true;
	}
</script>

{#if submit}
	<AddKrakenLoader {krakenApi} {krakenSk} {password} />
{:else}
	<div class="hero min-h bg-base-200">
		<div class="hero-content">
			<div class="max-w-md">
				<h1 class="text-4xl text-primary font-bold">Add Kraken API Key</h1>
				<div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100 mt-4">
					<form class="card-body">
						<div class="flex flex-col w-full justify-center items-center">
							{#if errMsg}
								<div class="text-center lg:text-left">
									<span class="text-red-600">{errMsg}</span>
								</div>
							{/if}
							<label>
								<span class="label-text">Kraken API Key</span>
								<br />
								<input
									type="text"
									placeholder="Kraken API Key"
									class="input input-bordered"
									bind:value={krakenApi}
									required
								/>
							</label>
							<label>
								<span class="label-text">Kraken Private Key</span>
								<br />
								<input
									type="text"
									placeholder="Kraken Private Key"
									class="input input-bordered"
									bind:value={krakenSk}
									required
								/>
							</label>
							<label>
								<span class="label-text">Password</span>
								<br />
								{#if show}
									<input
										type="text"
										placeholder="password"
										class="input input-bordered"
										bind:value={password}
										required
									/>
								{:else}
									<input
										type="password"
										placeholder="password"
										class="input input-bordered"
										bind:value={password}
										required
									/>
								{/if}
								<br />
							</label>
							<div class="label my-0 py-0">
								<a
									href="https://support.kraken.com/hc/en-us/articles/360000919966-How-to-create-an-API-key#1"
									class="label-text-alt link link-hover">How to get API keys?</a
								>
							</div>
						</div>
						<div class="form-control mt-6">
							<button class="btn btn-primary" on:click={addKeys}>Add Keys</button>
						</div>
					</form>
					<button class="btn btn-ghost text-primary btn-xs" on:click={() => (show = !show)}
						>Show Password</button
					>
				</div>
			</div>
		</div>
	</div>
{/if}
