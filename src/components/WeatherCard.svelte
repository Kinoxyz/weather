<script lang="ts">
  export let data: any; // Weather data
  let widgets = [1, 2, 3]; // Widgets for the top row
  let largeWidget = 4; // Large widget for the bottom
</script>

<div class="card">
  <div class="grid">
    {#each widgets as widget (widget)}
      <div class="widget">
        <!-- Create widgets here -->
      </div>
    {/each}
  </div>
  <div class="large-widget">
    <p>
      Current temperature : {Object.keys(data).length === 0
        ? 0
        : JSON.stringify(data.current.temperature_2m)}
    </p>
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

  .widget {
    background-color: #3498db;
    border: 1px solid #2979ac;
    border-radius: 5px;
    padding: 10px;
    text-align: center;
    color: #fff;
    font-weight: bold;
    margin-bottom: 5px;
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
