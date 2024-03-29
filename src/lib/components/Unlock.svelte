<script>
	import { getUser } from '$lib';
	import { onMount } from 'svelte';
	import Login from '$lib/components/Login.svelte';
	import UnlockLoader from '$lib/loaders/UnlockLoader.svelte';

	export let err;

	let user = '';

	let password = '';
	let show = false;
	let errMsg = err ? err : '';
	let submit = false;

	function unlock() {
		if (password) {
			submit = true;
		} else {
			errMsg = 'Invalid input';
		}
	}

	onMount(async () => {
		user = await getUser();
	});
</script>

{#if user}
	{#if submit}
		<UnlockLoader {password} user="" msg="" />
	{:else}
		<div>
			<div class="hero min-h-fit bg-base-200" style="height: 44rem;">
				<div class="hero-content flex-col">
					<div class="text-center text-primary lg:text-left">
						<h1 class="text-5xl font-bold">Unlock</h1>
					</div>
					<div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
						<form class="card-body">
							<div class="flex flex-col w-full justify-center items-center">
								{#if errMsg}
									<div class="text-center lg:text-left">
										<span class="text-red-600">{errMsg}</span>
									</div>
								{/if}
								<div class="h-12 w-12 rounded-full border border-primary m-2">
									<minidenticon-svg username={user.username}></minidenticon-svg>
								</div>
								<div class="max-w-md">
									<h1 class="text-3xl text-primary font-bold">
										{user.username.charAt(0).toUpperCase() + user.username.slice(1)}
									</h1>
								</div>
							</div>
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
								<button class="btn btn-primary" on:click={unlock}>Unlock</button>
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
{:else}
	<Login uName="" msg="" err="" />
{/if}
