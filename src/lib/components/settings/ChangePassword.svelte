<script>
	import ChangePasswordLoader from '$lib/loaders/ChangePasswordLoader.svelte';

	export let err;

	let oldPassword = '';
	let newPassword = '';
	let passwordCheck = '';
	let show = false;
	let errMsg = err ? err : '';
	let submit = false;

	function changePass() {
		errMsg = '';
		if (oldPassword && newPassword && passwordCheck) {
			if (oldPassword != newPassword) {
				if (newPassword.length < 10 || oldPassword.length < 10) {
					errMsg = 'Password must be greater than 10 characters';
				} else {
					if (newPassword != passwordCheck) {
						errMsg = 'Passwords do not match';
					} else {
						submit = true;
					}
				}
			} else {
				errMsg = 'New password must be different from old password';
			}
		} else {
			errMsg = 'Invalid input';
		}
	}
</script>

{#if submit}
	<ChangePasswordLoader {oldPassword} {newPassword} />
{:else}
	<div class="hero min-h bg-base-200">
		<div class="hero-content">
			<div class="max-w-md">
				<h1 class="text-5xl text-primary font-bold">Change Password</h1>
				<div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100 mt-4">
					<form class="card-body">
						<div class="flex flex-col w-full justify-center items-center text-primary">
							{#if errMsg}
								<div class="text-center lg:text-left">
									<span class="text-red-600">{errMsg}</span>
								</div>
							{/if}
							<label>
								<span class="label-text">Old Password</span>
								<br />
								{#if show}
									<input
										type="text"
										placeholder="Old Password"
										class="input input-bordered"
										bind:value={oldPassword}
										required
									/>
								{:else}
									<input
										type="password"
										placeholder="Old Password"
										class="input input-bordered"
										bind:value={oldPassword}
										required
									/>
								{/if}
								<br />
							</label>
							<label>
								<span class="label-text">New Password</span>
								<br />
								{#if show}
									<input
										type="text"
										placeholder="New Password"
										class="input input-bordered"
										bind:value={newPassword}
										required
									/>
								{:else}
									<input
										type="password"
										placeholder="New Password"
										class="input input-bordered"
										bind:value={newPassword}
										required
									/>
								{/if}
								<br />
							</label>
							<label>
								<span class="label-text">Repeat New Password</span>
								<br />
								{#if show}
									<input
										type="text"
										placeholder="Repeat New Password"
										class="input input-bordered"
										bind:value={passwordCheck}
										required
									/>
								{:else}
									<input
										type="password"
										placeholder="Repeat New Password"
										class="input input-bordered"
										bind:value={passwordCheck}
										required
									/>
								{/if}
								<br />
							</label>
						</div>
						<div class="form-control mt-6">
							<button class="btn btn-primary" on:click={changePass}>Change Password</button>
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
