<script lang="ts">
    export let data: any;
    export let temperatureUnit: String;
    export let wmoCodeDescription: String;
</script>

<div class="widget">
    {#if Object.keys(data).length !== 0}
        <h1>
            {JSON.stringify(data.current.temperature_2m) + temperatureUnit}
        </h1>
        <br />
        {wmoCodeDescription}
    {/if}
</div>

<style>
    .widget {
        background-color: #3498db;
        border: 1px solid #2979ac;
        border-radius: 5px;
        padding: 10px;
        text-align: center;
        color: #fff;
        margin-bottom: 5px;
    }
</style>
