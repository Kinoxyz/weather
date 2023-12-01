<script lang="ts">
  export let fetchWeatherData: (location: string) => Promise<void>;
  export let locationInput: string;

  async function handleSubmit() {
    fetchWeatherData(locationInput);
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={handleSubmit}>
    <input
      id="location-input"
      placeholder="Enter a location..."
      bind:value={locationInput}
    />
    <button type="submit">Submit</button>
  </form>
</div>
