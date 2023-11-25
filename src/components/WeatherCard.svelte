<script lang="ts">
  import type { CurrentWeatherResponse } from "../bindings/CurrentWeatherResponse";
  import CurrentWeatherConditionWidget from "./widgets/CurrentWeatherConditionWidget.svelte";
  import DailyWeatherConditionWidget from "./widgets/DailyWeatherConditionWidget.svelte";

	import TomorrowWeatherConditionWidget from "./widgets/TomorrowWeatherConditionWidget.svelte";
  export let data: CurrentWeatherResponse; // Weather data
  export let wmoCodeDescription: String;
</script>

<div class="card">
  <div class="grid">
    <CurrentWeatherConditionWidget {data} {wmoCodeDescription} />
    <DailyWeatherConditionWidget {data} />
    <TomorrowWeatherConditionWidget {data} />
  </div>
  <div class="large-widget">
    <p />
  </div>
</div>

<style>
  .card {
    padding: 20px;
    border-radius: 6px;
    width: 600px;
    height: 400px;
    min-height: 400px;
    box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.1);
    margin-top: 20px;
    margin-bottom: 20px;
    display: flex;
    flex-direction: column;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-gap: 10px;
    flex: 1;
  }

  .large-widget {
    background-color: #3498db;
    border: 1px solid #2979ac;
    border-radius: 5px;
    padding: 10px;
    text-align: center;
    color: #fff;
    font-weight: bold;
    flex: 1;
  }
</style>
