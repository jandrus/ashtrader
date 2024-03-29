<script>
	import * as d3 from 'd3';

	export let xScale;
	export let innerHeight;
	export let hoveredPoint;
	export let label;
	export let interval;

	function getFormat() {
		switch (interval) {
			case '1':
				return d3.timeFormat('%I:%M %p');
			case '5':
				return d3.timeFormat('%I:%M %p');
			case '30':
				return d3.timeFormat('%d%b %I:%M %p');
			case '60':
				return d3.timeFormat('%d%b %I:%M %p');
			case '240':
				return d3.timeFormat('%d%b %I:%M %p');
			case '1440':
				return d3.timeFormat('%d%b%Y');
			case '10080':
				return d3.timeFormat('%d%b%Y');
			case '21600':
				return d3.timeFormat('%d%b%Y');
			default:
				return d3.timeFormat('%d%b%y %I:%M %p');
		}
	}

	const formatXLabel = getFormat();

	$: [xMin, xMax] = xScale.range();

	$: ticks = xScale.ticks(3);
</script>

<g transform={`translate(0 ${innerHeight})`}>
	<line x1={xMin} x2={xMax} y1={0} y2={0} stroke="#bdc3c7" />
	{#each ticks as tick}
		<g transform={`translate(${xScale(tick)} 0)`}>
			<line y1={0} y2={6} stroke="#bdc3c7" />
			<text y={10} dy="0.8em" text-anchor="middle" fill={hoveredPoint ? '#bdc3c7' : '#282828'}>
				{formatXLabel(tick)}
			</text>
		</g>
	{/each}
	<text x={xScale.range()[1] / 2} text-anchor="middle" y={45} fill="#282828">
		{label}
	</text>
</g>
