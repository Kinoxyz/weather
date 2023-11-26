<script lang="ts">
    import { currentLocationName, currentLocationCountry } from '../stores';

    function getLocationString(name: String, country: String): String {
        if (!name || !country) {
          return "<br>"
        } else {
          return `<strong>Current location: ${name}, ${country}</strong>`
        }
    }
</script>

<div class=current-location>
    {@html getLocationString($currentLocationName, $currentLocationCountry) }
</div>

<style>
    .current-location {
      margin-top: 5px;
      margin-bottom: 5px;
    }
</style>