<script lang="ts">
    import type {WeatherResponse} from "../../bindings/WeatherResponse";
    import {temperatureUnit} from "../../stores";
    import {getTemperatureString} from "../../models/Temperature";

    export let data: WeatherResponse;
</script>

<div class="widget">
    {#if Object.keys(data).length !== 0}
        <h2>Tomorrow</h2>
        <div class="temperature-row">
            <div class="label">High:</div>
            <div class="temperature-text">
                {getTemperatureString(data.daily.temperature_2m_max[1], $temperatureUnit)}
            </div>
        </div>
        <br/>
        <div class="temperature-row">
            <div class="label">Low:</div>
            <div class="temperature-text">
                {getTemperatureString(data.daily.temperature_2m_min[1], $temperatureUnit)}
            </div>
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
        position: relative;    
    }


    .temperature-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
    margin-inline: 20px; 
    }

    .label {
        font-size: 1em;
        font-weight: bold;
    }

    .temperature-text {
        font-size: 2em;
        font-weight: bold;
    }
</style>


