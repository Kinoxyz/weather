<script lang="ts">
    import LocationInputField from "./components/LocationInputField.svelte";
    import WeatherCard from "./components/WeatherCard.svelte";
    import Footer from "./components/Footer.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import SettingsButton from "./components/SettingsButton.svelte";
	import CurrentLocationDisplay from "./components/CurrentLocationDisplay.svelte";
    import { currentLocationName, currentLocationCountry } from './stores';
	import type { WeatherData } from "./bindings/WeatherData";

    let weatherResponse: any = {};
    let wmoCodeDescription = "";
    let locationInput = "";

    async function fetchWeatherData(location: string) {
        let weatherData: WeatherData = await invoke("get_weather_data", {location});
        console.log(weatherData);

        $currentLocationName = weatherData?.location.name;
        $currentLocationCountry = weatherData?.location.country;
        weatherResponse = weatherData?.weather_response;

        let wmoCode = weatherResponse.current.weathercode;
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
            <WeatherCard data={weatherResponse} {wmoCodeDescription}/>
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
