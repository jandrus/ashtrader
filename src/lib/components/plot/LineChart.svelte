<script>
	import * as d3 from 'd3';
	import { onMount } from 'svelte';
	import { getPortfolio, getUser, unlocked } from '$lib';
	import Line from '$lib/components/plot/Line.svelte';
	import XAxis from '$lib/components/plot/XAxis.svelte';
	import GridLines from '$lib/components/plot/GridLines.svelte';
	import Crosshair from '$lib/components/plot/Crosshair.svelte';
	import Point from '$lib/components/plot/Point.svelte';
	import Buy from '$lib/components/Buy.svelte';
	import Sell from '$lib/components/Sell.svelte';

	export let data;
	export let asset;
	export let interval;

	let user;
	let isUnlocked;
	let portfolio = null;
	let hoveredPoint = null;

	const margin = {
		top: 50,
		right: 60,
		bottom: 50,
		left: 155
	};

	let yLabel = 'Price (' + asset.quote + ')';

	let width = 100;
	$: height = 0.4 * width;

	$: innerWidth = width - margin.left - margin.right;
	$: innerHeight = height - margin.top - margin.bottom;

	const xAccessor = (d) => new Date(d.timestamp * 1000);
	const yAccessor = (d) => d.close;

	const bisectX = d3.bisector(xAccessor).left;

	$: xScale = d3.scaleLinear().domain(d3.extent(data, xAccessor)).range([0, innerWidth]);

	$: yScale = d3.scaleLinear().domain(d3.extent(data, yAccessor)).range([innerHeight, 0]).nice();

	$: xAccessorScaled = (d) => xScale(xAccessor(d));
	const yAccessorScaled = (d) => yScale(yAccessor(d));

	const handleMouseMove = (event) => {
		const xCoordinate = xScale.invert(event.offsetX - margin.left);
		const index = bisectX(data, xCoordinate);
		hoveredPoint = data[index - 1];
	};

	const handleMouseLeave = () => {
		hoveredPoint = null;
	};

	onMount(async () => {
		isUnlocked = await unlocked();
		user = await getUser();
		portfolio = await getPortfolio();
	});
</script>

<div class="wrapper" bind:clientWidth={width}>
	<svg
		role="img"
		aria-label="chart"
		{width}
		{height}
		on:mousemove={handleMouseMove}
		on:mouseleave={handleMouseLeave}
	>
		<g transform={`translate(${margin.left}, ${margin.top})`}>
			<XAxis {xScale} {innerHeight} {hoveredPoint} label="" {interval} />
			<GridLines {yScale} {innerWidth} {hoveredPoint} label={yLabel} {asset} />
			<Line {data} {xAccessorScaled} {yAccessorScaled} />
			{#if hoveredPoint}
				<Crosshair
					xAccessorScaled={xAccessorScaled(hoveredPoint)}
					yAccessorScaled={yAccessorScaled(hoveredPoint)}
					xLabel={xAccessor(hoveredPoint)}
					yLabel={yAccessor(hoveredPoint)}
					{innerHeight}
					{interval}
					{asset}
				/>
				<Point
					x={xAccessorScaled(hoveredPoint)}
					y={yAccessorScaled(hoveredPoint)}
					color="#dc267f"
				/>
			{/if}
		</g>
	</svg>
</div>
<h1 class="text-xl text-primary mb-2">
	Current Price: {data[data.length - 1].close.toFixed(asset.base_decimals)}
	{asset.quote}
</h1>
{#if user}
	{#if user.kraken_auth}
		{#if isUnlocked}
			{#if portfolio}
				<div class="flex space-x-4">
					<Buy {data} {asset} {portfolio} />
					<Sell {data} {asset} {portfolio} />
				</div>
			{:else}
				<div class="flex space-x-4">
					<div class="skeleton w-96 h-64"></div>
					<div class="skeleton w-96 h-64"></div>
				</div>
			{/if}
		{:else}
			<a class="btn btn-outline btn-error" href="/unlock">
				<h1 class="text-xl">Unlock account to trade</h1>
			</a>
		{/if}
	{:else}
		<a class="btn btn-outline btn-error" href="/addKraken">
			<h1 class="text-xl text-error">Add Kraken API Keys to trade</h1>
		</a>
	{/if}
{:else}
	<a class="btn btn-outline btn-error" href="/login">
		<h1 class="text-xl text-error">Login to trade</h1>
	</a>
{/if}

<style>
	.wrapper {
		position: relative;
		width: 100%;
		max-width: 800px;
	}
</style>
