<script lang="ts">
    import { currentLocationName, currentLocationCountry } from '../stores';

    function getLocationString(name: String, country: String): String {
        if (!name || !country) {
          return " "
        } else {
          return `Current location: ${name}, ${country}`
        }
    }
</script>

<div>
    <h2>{getLocationString($currentLocationName, $currentLocationCountry)}</h2>
</div>
