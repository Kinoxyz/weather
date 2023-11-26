import {writable} from "svelte/store";
import {Temperature} from "./models/Temperature";

export const temperatureUnit = writable(Temperature.Celsius)