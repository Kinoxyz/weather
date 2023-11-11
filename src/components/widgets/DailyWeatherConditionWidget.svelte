<script lang="ts">
    export let data: any;
    export let temperatureUnit: String;
</script>

<div class="widget">
    <h4>High:</h4>
    <h1>
        {JSON.stringify(data.daily.temperature_2m_max[0]) + temperatureUnit}
    </h1>
    <h4>Low:</h4>
    <h1>
        {JSON.stringify(data.daily.temperature_2m_min[0]) + temperatureUnit}
    </h1>
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
