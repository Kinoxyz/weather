<script lang="ts">
    import LocationInputField from "./components/LocationInputField.svelte";
    import WeatherCard from "./components/WeatherCard.svelte";
    import Footer from "./components/Footer.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import SettingsButton from "./components/SettingsButton.svelte";
	import CurrentLocationDisplay from "./components/CurrentLocationDisplay.svelte";
    import { currentLocationName, currentLocationCountry } from './stores';
	import type { WeatherData } from "./bindings/WeatherData";

    let weatherData: any = {};
    let wmoCodeDescription = "";
    let locationInput = "";

    async function fetchWeatherData(location: string) {
        let retrievedWeatherData: WeatherData = await invoke("get_weather_data", {location});
        console.log(retrievedWeatherData);

        $currentLocationName = retrievedWeatherData?.location.name;
        $currentLocationCountry = retrievedWeatherData?.location.country;
        weatherData = retrievedWeatherData;

        let wmoCode = retrievedWeatherData.current.weathercode;
        wmoCodeDescription = await invoke("get_wmo_code_description", {
            code: wmoCode,
        });
    }
</script>

<div class="flex-container">
    <main class="container">
        <SettingsButton/>
        <div class="row">
            <LocationInputField {fetchWeatherData} {locationInput}/>
        </div>
        <div class="row">
            <CurrentLocationDisplay/>
        </div>
        <div class="row">
            <WeatherCard data={weatherData} {wmoCodeDescription}/>
        </div>
    </main>
    <Footer/>
</div>

<style>
    .flex-container {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }

    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        flex-grow: 1;
        margin: auto;
    }
</style>
