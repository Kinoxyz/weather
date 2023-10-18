<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let data = {};
  let location = "";

  async function fetchWeatherData() {
    data = await invoke("get_weather_data", { location });
    console.log(data);
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={fetchWeatherData}>
    <input
      id="location-input"
      placeholder="Enter a location..."
      bind:value={location}
    />
    <button type="submit">Submit</button>
  </form>
</div>
