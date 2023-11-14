<script>
  import { Temperature } from "../models/Temperature";
  import { temperatureUnit } from "../stores";
</script>

<div
  on:click={() => temperatureUnit.set(Temperature.Fahrenheit)}
  on:dblclick={() => temperatureUnit.set(Temperature.Celsius)}
  class="settings-container"
>
  <img src="/settings_material.svg" alt="Settings icon" />
</div>

<style>
  .settings-container {
    display: flex;
    align-self: flex-end;
    padding: 1em;
    padding-bottom: 0%;
  }

  /* filter from https://isotropic.co/tool/hex-color-to-css-filter/ */
  img {
    filter: invert(100%) sepia(100%) saturate(2%) hue-rotate(26deg)
      brightness(106%) contrast(101%);
  }
</style>
