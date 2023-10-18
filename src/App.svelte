<script lang="ts">
  import LocationInputField from "./components/LocationInputField.svelte";
  import WeatherCard from "./components/WeatherCard.svelte";
  import Footer from "./components/Footer.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let data = {};
  let location = "";

  async function fetchWeatherData(location: string) {
    data = await invoke("get_weather_data", { location });
    console.log(data);
  }
</script>

<div class="flex-container">
  <main class="container">
    <div class="row">
      <LocationInputField {fetchWeatherData} {location} />
    </div>
    <div class="row">
      <WeatherCard {data} />
    </div>
  </main>
  <Footer />
</div>

<style>
  .flex-container {
    display: flex;
    flex-direction: column;
    min-height: 90vh;
  }
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex-grow: 1;
  }
</style>
