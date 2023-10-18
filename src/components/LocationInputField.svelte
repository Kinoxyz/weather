<script lang="ts">
  export let fetchWeatherData: (location: string) => Promise<void>;
  export let location: string;

  async function handleSubmit() {
    fetchWeatherData(location);
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={handleSubmit}>
    <input
      id="location-input"
      placeholder="Enter a location..."
      bind:value={location}
    />
    <button type="submit">Submit</button>
  </form>
</div>
