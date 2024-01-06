<script lang="ts">
	import type { WeatherData } from '../bindings/WeatherData';

	export let data: WeatherData;
	export let weatherDataError: String;

	function getLocationString(name: String, country: String): String {
		return `Current location: ${name}, ${country}`;
	}
</script>

<div class="current-location">
	{#if weatherDataError}
		<strong>{weatherDataError}</strong>
	{:else if Object.keys(data).length !== 0}
		<strong>{getLocationString(data.location.name, data.location.country)}</strong>
	{:else}
		<br />
	{/if}
</div>

<style>
	.current-location {
		margin-top: 5px;
		margin-bottom: 5px;
	}
</style>
