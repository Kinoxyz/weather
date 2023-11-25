<script lang="ts">
    import LocationInputField from "./components/LocationInputField.svelte";
    import WeatherCard from "./components/WeatherCard.svelte";
    import Footer from "./components/Footer.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import SettingsButton from "./components/SettingsButton.svelte";

    let weatherData: any = {};
    let wmoCodeDescription = "";
    let location = "";

    async function fetchWeatherData(location: string) {
        weatherData = await invoke("get_weather_data", {location});
        console.log(weatherData);
        let wmoCode = weatherData?.current.weathercode;
        wmoCodeDescription = await invoke("get_wmo_code_description", {
            code: wmoCode,
        });
    }
</script>

<div class="flex-container">
    <main class="container">
        <SettingsButton/>
        <div class="row">
            <LocationInputField {fetchWeatherData} {location}/>
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
