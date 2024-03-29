<script>
	export let xAccessorScaled;
	export let yAccessorScaled;
	export let xLabel;
	export let yLabel;
	export let innerHeight;
	export let interval;
	export let asset;

	import * as d3 from 'd3';

	function getFloatFormat() {
		switch (asset.quote) {
			case 'USD':
				return '$,.' + asset.quote_decimals + 'f';
			default:
				return ',.' + asset.quote_decimals + 'f';
		}
	}

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

	const formatYLabel = d3.format(getFloatFormat());
	const formatXLabel = getFormat();
</script>

<g transform={`translate(${xAccessorScaled} 0)`}>
	<text class="text" y={-20} dy="0.8em" text-anchor="middle">
		{formatXLabel(xLabel)}
	</text>
	<line class="line" x1={0} x2={0} y1={yAccessorScaled} y2={innerHeight} />
</g>

<g transform={`translate(0 ${yAccessorScaled})`}>
	<text class="text" dx={-10} dy="0.34em" text-anchor="end">
		{formatYLabel(yLabel)}
	</text>
	<line class="line" x1={xAccessorScaled} x2={0} y1={0} y2={0} />
</g>

<style>
	.text {
		fill: #dc267f;
	}
	.line {
		stroke: #dc267f;
		stroke-width: 1;
	}
</style>
