<script lang="ts">
    import type {CurrentWeatherResponse} from "../../bindings/CurrentWeatherResponse";
    import {temperatureUnit} from "../../stores";
    import {getTemperatureString} from "../../models/Temperature";

    export let data: CurrentWeatherResponse;
</script>

<div class="widget">
    {#if Object.keys(data).length !== 0}
        <h4>High:</h4>
        <h1>
            {getTemperatureString(data.daily.temperature_2m_max[0], $temperatureUnit)}
        </h1>
        <h4>Low:</h4>
        <h1>
            {getTemperatureString(data.daily.temperature_2m_min[0], $temperatureUnit)}
        </h1>
    {/if}
</div>

<style>
    .widget {
        background-color: #3498db;
        border: 1px solid #2979ac;
        border-radius: 5px;
        padding: 5px;
        text-align: center;
        color: #fff;
        margin-bottom: 5px;
    }
</style>
