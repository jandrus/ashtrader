<script>
	import { onMount } from 'svelte';
	import { themeChange } from 'theme-change';

	const themes = [
		'light',
		'dark',
		'cupcake',
		'bumblebee',
		'emerald',
		'corporate',
		'synthwave',
		'retro',
		'cyberpunk',
		'valentine',
		'halloween',
		'garden',
		'forest',
		'aqua',
		'lofi',
		'pastel',
		'fantasy',
		'wireframe',
		'black',
		'luxury',
		'dracula',
		'cmyk',
		'autumn',
		'business',
		'acid',
		'lemonade',
		'night',
		'coffee',
		'winter',
		'dim',
		'nord',
		'sunset'
	];

	onMount(() => {
		themeChange(false);
	});
</script>

<div class="hero min-h bg-base-200">
	<div class="hero-content">
		<div class="max-w">
			<h1 class="text-5xl mb-3 text-primary font-bold">Change Theme</h1>
			<!-- <div class="dropdown">
				 <div tabindex="0" role="button" class="btn">
				 Theme
				 <svg
				 width="12px"
				 height="12px"
				 class="h-2 w-2 fill-current opacity-60 inline-block"
				 xmlns="http://www.w3.org/2000/svg"
				 viewBox="0 0 2048 2048"
				 >
				 <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z" />
				 </svg>
				 </div>
				 <ul
				 class="dropdown-content z-[1] p-2 shadow-2xl bg-base-300 rounded-box w-52 flex flex-col gap-y-2"
				 >
				 <li>
				 <input
				 type="radio"
				 name="theme-dropdown"
				 class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
				 aria-label={theme}
				 value={theme}
				 />
				 </li>
				 </ul>
			     </div> -->
			<!-- <div class="rounded-box grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5"> -->
			<div class="rounded-box grid grid-cols-5 gap-3">
				{#each themes as theme}
					<button data-set-theme={theme} data-act-class="ACTIVECLASS">
						<div
							class="border-base-content/20 hover:border-base-content/40 overflow-hidden rounded-lg border outline outline-2 outline-offset-2 outline-transparent"
							data-act-class="!outline-base-content"
						>
							<div
								data-theme={theme}
								class="bg-base-100 text-base-content w-full cursor-pointer font-sans"
							>
								<div class="grid grid-cols-4 grid-rows-3">
									<div class="bg-base-200 col-start-1 row-span-2 row-start-1"></div>
									<div class="bg-base-300 col-start-1 row-start-3"></div>
									<div
										class="bg-base-100 col-span-4 col-start-2 row-span-3 row-start-1 flex flex-col gap-1 p-2"
									>
										<div class="font-bold">{theme}</div>
										<div class="flex flex-wrap gap-1">
											<div
												class="bg-primary flex aspect-square w-5 items-center justify-center rounded lg:w-6"
											>
												<div class="text-primary-content text-sm font-bold">A</div>
											</div>
											<div
												class="bg-secondary flex aspect-square w-5 items-center justify-center rounded lg:w-6"
											>
												<div class="text-secondary-content text-sm font-bold">A</div>
											</div>
											<div
												class="bg-accent flex aspect-square w-5 items-center justify-center rounded lg:w-6"
											>
												<div class="text-accent-content text-sm font-bold">A</div>
											</div>
											<!-- <div
                                            class="bg-neutral flex aspect-square w-5 items-center justify-center rounded lg:w-6"
                                            >
                                            <div class="text-neutral-content text-sm font-bold">A</div>
                                            </div> -->
										</div>
									</div>
								</div>
							</div>
						</div>
					</button>
				{/each}
			</div>
		</div>
	</div>
</div>
