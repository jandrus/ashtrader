<script>
	import { ok } from '$lib';
	import { onMount } from 'svelte';
	import LoginLoader from '$lib/loaders/LoginLoader.svelte';

	export let uName;
	export let msg;
	export let err;

	let username = uName ? uName : '';
	let password = '';
	let show = false;
	let errMsg = err ? err : '';
	let submit = false;

	function registerUser() {
		if (username && password) {
			submit = true;
		} else {
			errMsg = 'Invalid input';
		}
	}

	onMount(async () => {
		if (msg) {
			await ok(msg);
		}
	});
</script>

{#if submit}
	<LoginLoader {username} {password} />
{:else}
	<div>
		<div class="hero min-h-fit bg-base-200" style="height: 44rem;">
			<div class="hero-content flex-col">
				<div class="text-center text-primary lg:text-left">
					<h1 class="text-5xl font-bold">Login</h1>
				</div>
				<div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
					<form class="card-body">
						<div class="flex flex-col w-full justify-center items-center">
							{#if msg}
								<div class="text-center lg:text-left">
									<span class="text-blue-600">{msg}</span>
								</div>
								<br />
							{/if}
							{#if username}
								<div class="h-12 w-12 rounded-full border m-2">
									<minidenticon-svg {username}></minidenticon-svg>
								</div>
							{:else}
								<div class="h-12 w-12 rounded-full m-2"></div>
							{/if}
							{#if errMsg}
								<div class="text-center lg:text-left">
									<span class="text-red-600">{errMsg}</span>
								</div>
							{/if}
						</div>
						<label>
							<span class="label-text">Username</span>
							<br />
							<input
								type="username"
								placeholder="username"
								class="input input-bordered"
								bind:value={username}
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
							<a href="/register" class="label-text-alt link link-hover">Register?</a>
						</div>
						<div class="form-control mt-6">
							<button class="btn btn-primary" on:click={registerUser}>Login</button>
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
