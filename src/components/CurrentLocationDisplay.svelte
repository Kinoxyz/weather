<script lang="ts">
    import type {WeatherData} from "../bindings/WeatherData";

    export let data: WeatherData;

    function getLocationString(name: String, country: String): String {
        return `Current location: ${name}, ${country}`
    }
</script>

<div class=current-location>
    {#if Object.keys(data).length === 0}
        <br>
    {/if}

    {#if Object.keys(data).length !== 0}
        <strong>{getLocationString(data.location.name, data.location.country)}</strong>
    {/if}
    
</div>

<style>
    .current-location {
      margin-top: 5px;
      margin-bottom: 5px;
    }
</style>