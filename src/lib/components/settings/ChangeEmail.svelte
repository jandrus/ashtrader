<script>
	import { validEmail } from '$lib';
	import ChangeEmailLoader from '$lib/loaders/ChangeEmailLoader.svelte';

	export let err;

	let password = '';
	let email = '';
	let show = false;
	let errMsg = err ? err : '';
	let submit = false;

	function changeEmail() {
		errMsg = '';
		if (email && password) {
			if (validEmail(email)) {
				submit = true;
			} else {
				errMsg = 'Invalid email';
			}
		} else {
			errMsg = 'Invalid input';
		}
	}
</script>

{#if submit}
	<ChangeEmailLoader {password} {email} />
{:else}
	<div class="hero min-h bg-base-200">
		<div class="hero-content">
			<div class="max-w-md">
				<h1 class="text-5xl text-primary font-bold">Change Email</h1>
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
										placeholder="Password"
										class="input input-bordered"
										bind:value={password}
										required
									/>
								{:else}
									<input
										type="password"
										placeholder="Password"
										class="input input-bordered"
										bind:value={password}
										required
									/>
								{/if}
								<br />
							</label>
							<label>
								<span class="label-text">New Email</span>
								<br />
								<input
									type="text"
									placeholder="New Email"
									class="input input-bordered"
									bind:value={email}
									required
								/>
								<br />
							</label>
						</div>
						<div class="form-control mt-6">
							<button class="btn btn-primary" on:click={changeEmail}>Change Email</button>
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
