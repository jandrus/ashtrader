<script>
	import { validEmail } from '$lib';
	import RegistrationLoader from '$lib/loaders/RegistrationLoader.svelte';

	export let err;

	let username = '';
	let email = 'test@test.com';
	let password = '';
	let passwordCheck = '';
	let show = false;
	let errMsg = err ? err : '';
	let submit = false;

	function registerUser() {
		errMsg = '';
		if (username && email && password && passwordCheck) {
			if (validEmail(email)) {
				if (password.length < 10) {
					errMsg = 'Password must be greater than 10 characters';
				} else {
					if (password != passwordCheck) {
						errMsg = 'Passwords do not match';
					} else {
						submit = true;
					}
				}
			} else {
				errMsg = 'Invalid email';
			}
		} else {
			errMsg = 'Invalid input';
		}
	}
</script>

{#if submit}
	<RegistrationLoader {username} {email} {password} />
{:else}
	<div>
		<div class="hero min-h-fit bg-base-200" style="height: 44rem;">
			<div class="hero-content flex-col">
				<div class="text-center lg:text-left">
					<h1 class="text-5xl text-primary font-bold">Register</h1>
				</div>
				<div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
					<form class="card-body">
						<div class="flex flex-col w-full justify-center items-center">
							{#if username}
								<div class="h-12 w-12 rounded-full border border-primary m-2">
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
							<!-- <label>
								 <span class="label-text">Email</span>
								 <br />
								 <input
								 type="email"
								 placeholder="email"
								 class="input input-bordered"
								 bind:value={email}
								 required
								 />
								 </label> -->
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
							<label>
								<span class="label-text">Repeat Password</span>
								<br />
								{#if show}
									<input
										type="text"
										placeholder="repeat password"
										class="input input-bordered"
										bind:value={passwordCheck}
										required
									/>
								{:else}
									<input
										type="password"
										placeholder="repeat password"
										class="input input-bordered"
										bind:value={passwordCheck}
										required
									/>
								{/if}
								<br />
							</label>
						</div>
						<div class="label my-0 py-0">
							<a href="/login" class="label-text-alt link link-hover">Login instead?</a>
						</div>
						<div class="form-control mt-6">
							<button class="btn btn-primary" on:click={registerUser}>Register</button>
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
