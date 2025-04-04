<script lang="ts">
    import LocationInputField from "./components/LocationInputField.svelte";
    import WeatherCard from "./components/WeatherCard.svelte";
    import Footer from "./components/Footer.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import SettingsButton from "./components/SettingsButton.svelte";
	import CurrentLocationDisplay from "./components/CurrentLocationDisplay.svelte";

    let weatherData: any = {};
    let locationInput = "";
    let weatherDataError = "";

    async function fetchWeatherData(location: string) {
        weatherData = {};
        weatherDataError="";
        try {
            weatherData = location.trim().length ? await invoke("get_weather_data", {location}) : {};
        } catch (err: any) {
            weatherDataError = err
        }
    }
</script>

<div class="flex-container">
    <main class="container">
        <SettingsButton/>
        <div class="row">
            <LocationInputField {fetchWeatherData} {locationInput}/>
        </div>
        <div class="row">
            <CurrentLocationDisplay data={weatherData} weatherDataError={weatherDataError}/>
        </div>
        <div class="row">
            <WeatherCard data={weatherData}/>
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
