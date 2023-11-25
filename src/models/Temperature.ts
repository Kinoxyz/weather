
export enum Temperature {
  Celsius = "°C",
  Fahrenheit = "°F",
}

function convertToFahrenheitString(temperature: number, digits?: number): string {
 return (temperature * 1.8 + 32).toFixed(digits);
}

export function getTemperatureString(temperature: number, unit: Temperature): string {
  switch (unit) {
    case Temperature.Celsius:
      return temperature.toString() + Temperature.Celsius;
    case Temperature.Fahrenheit:
      return convertToFahrenheitString(temperature, 1) + Temperature.Fahrenheit;
  }
}
