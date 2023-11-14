<script>
	import { Temperature } from '../models/Temperature';
	import Modal from './Modal.svelte';
	import { temperatureUnit } from '../stores';

	let unit = $temperatureUnit;

	function saveSettings() {
		temperatureUnit.set(unit)
	}

	let showModal = false;

</script>

<button
	on:click={() => (showModal = true)}
	class='settings-container'
>
	<img src='/settings_material.svg' alt='Settings icon' />
</button>

<Modal closeButtonText='Save settings' bind:showModal onCloseFunction={saveSettings}>
	<h2 slot='header'>
		Settings
	</h2>

	Temperature:
	<label>
		<input type='radio' bind:group={unit} value={Temperature.Celsius} />
		Celsius
	</label>
	<label>
		<input type='radio' bind:group={unit} value={Temperature.Fahrenheit} />
		Fahrenheit
	</label>
</Modal>

<style>
    /* filter from https://isotropic.co/tool/hex-color-to-css-filter/ */
    img {
        filter: invert(100%) sepia(100%) saturate(2%) hue-rotate(26deg) brightness(106%) contrast(101%);
    }

    button {
        margin-right: 1em;
        align-self: flex-end;
        width: fit-content;
    }

    button img {
        vertical-align: middle;
    }

</style>
