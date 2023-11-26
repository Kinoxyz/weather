<script lang="ts">
    import type {WeatherData} from "../../bindings/WeatherData";
    import {temperatureUnit} from "../../stores";
    import {getTemperatureString} from "../../models/Temperature.js";

    export let data: WeatherData;
    export let wmoCodeDescription: String;
</script>

<div class="widget">
    {#if Object.keys(data).length !== 0}
        <h2>Current</h2>
        <div class="temperature-text">
            {getTemperatureString(data.current.temperature_2m, $temperatureUnit)}
        </div>
        <br/>
        {wmoCodeDescription}
    {/if}
</div>

<style>
    .widget {
        background-color: #3498db;
        border: 1px solid #2979ac;
        border-radius: 5px;
        text-align: center;
        color: #fff;
        margin-bottom: 5px;
        display: flex;
        flex-direction: column;     
    }

    .temperature-text {
        font-size: 2em;
        font-weight: bold;
    }
</style>
