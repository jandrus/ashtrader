<script>
	import RemoveKrakenLoader from '$lib/loaders/RemoveKrakenLoader.svelte';

	export let err;

	let password = '';
	let show = false;
	let errMsg = err ? err : '';
	let submit = false;

	function rmKeys() {
		if (password) {
			submit = true;
		} else {
			errMsg = 'Invalid input';
		}
	}
</script>

{#if submit}
	<RemoveKrakenLoader {password} />
{:else}
	<div class="hero min-h bg-base-200">
		<div class="hero-content">
			<div class="max-w-md">
				<h1 class="text-4xl text-center text-primary font-bold">Remove Kraken Keys</h1>
				<div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100 mt-4">
					<form class="card-body">
						<div class="flex flex-col w-full justify-center items-center">
							{#if errMsg}
								<div class="text-center lg:text-left">
									<span class="text-red-600">{errMsg}</span>
								</div>
							{/if}
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
								<a href="/" class="label-text-alt link link-hover">Forgot password?</a>
							</div>
							<div class="form-control mt-6">
								<button class="btn btn-primary" on:click={rmKeys}>Remove Keys</button>
							</div>
							<p class="text-xs text-warning mt-2">
								Warning: This removes access to your Kraken account on this app.
							</p>
						</div>
					</form>
					<button class="btn btn-ghost btn-xs text-primary" on:click={() => (show = !show)}
						>Show Password</button
					>
				</div>
			</div>
		</div>
	</div>
{/if}
