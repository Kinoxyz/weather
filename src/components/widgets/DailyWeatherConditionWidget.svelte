<script lang="ts">
    import type {CurrentWeatherResponse} from "../../bindings/CurrentWeatherResponse";
    import {temperatureUnit} from "../../stores";
    import {getTemperatureString} from "../../models/Temperature";

    export let data: CurrentWeatherResponse;
</script>

<div class="widget">
    {#if Object.keys(data).length !== 0}
        <div class="label">Daily High:</div>
        <div class= "temperature-text">
            {getTemperatureString(data.daily.temperature_2m_max[0], $temperatureUnit)}
        </div>
        <div class="label">Daily Low:</div>
        <div class= "temperature-text">
            {getTemperatureString(data.daily.temperature_2m_min[0], $temperatureUnit)}
        </div>
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
    
    .label {
        font-size: 1em;
        font-weight: bold;
        margin-top: 20px;
        margin-bottom: 10px;
    }
</style>
