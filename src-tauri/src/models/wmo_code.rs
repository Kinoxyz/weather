use std::collections::HashMap;

pub fn get_wmo_code_description(code: i32) -> String {
    create_wmo_code_map()
        .get(&code)
        .unwrap_or(&String::from("Error retrieving WMO code description"))
        .to_owned()
}

// See https://www.nodc.noaa.gov/archive/arc0021/0002199/1.1/data/0-data/HTML/WMO-CODE/WMO4677.HTM
// Note: codes 00-49 indicate no precipitation at station at the time of observation,
// while codes 50-99 indicate precipitation at the time of observation.
fn create_wmo_code_map() -> HashMap<i32, String> {
    let mut wmo_code_map = HashMap::new();
    wmo_code_map.insert(
        0,
        "Cloud development not observed or not observable".to_string(),
    );
    wmo_code_map.insert(
        1,
        "Clouds generally dissolving or becoming less developed".to_string(),
    );
    wmo_code_map.insert(2, "State of sky on the whole unchanged".to_string());
    wmo_code_map.insert(3, "Clouds generally forming or developing".to_string());
    wmo_code_map.insert(4, "Visibility reduced by smoke, e.g. veldt or forest fires, industrial smoke or volcanic ashes".to_string());
    wmo_code_map.insert(5, "Haze".to_string());
    wmo_code_map.insert(6, "Widespread dust in suspension in the air, not raised by wind at or near the station at the time of observation".to_string());
    wmo_code_map.insert(7, "Dust or sand raised by wind at or near the station at the time of observation, but no well developed dust whirl(s) or sand whirl(s), and no duststorm or sandstorm seen".to_string());
    wmo_code_map.insert(8, "Well developed dust whirl(s) or sand whirl(s) seen at or near the station during the preceding hour or at the time ot observation, but no duststorm or sandstorm".to_string());
    wmo_code_map.insert(9, "Duststorm or sandstorm within sight at the time of observation, or at the station during the preceding hour".to_string());
    wmo_code_map.insert(10, "Mist".to_string());
    wmo_code_map.insert(11, "Patches".to_string());
    wmo_code_map.insert(12, "More or less continuous".to_string());
    wmo_code_map.insert(13, "Lightning visible, no thunder heard".to_string());
    wmo_code_map.insert(
        14,
        "Precipitation within sight, not reaching the ground or the surface of the sea".to_string(),
    );
    wmo_code_map.insert(15, "Precipitation within sight, reaching the ground or the surface of the sea, but distant, i.e. estimated to be more than 5 km from the station".to_string());
    wmo_code_map.insert(16, "Precipitation within sight, reaching the ground or the surface of the sea, near to, but not at the station".to_string());
    wmo_code_map.insert(
        17,
        "Thunderstorm, but no precipitation at the time of observation".to_string(),
    );
    wmo_code_map.insert(18, "Squalls".to_string());
    wmo_code_map.insert(19, "Funnel cloud(s)".to_string());
    wmo_code_map.insert(20, "Drizzle (not freezing) or snow grains".to_string());
    wmo_code_map.insert(21, "Rain (not freezing)".to_string());
    wmo_code_map.insert(22, "Snow".to_string());
    wmo_code_map.insert(23, "Rain and snow or ice pellets".to_string());
    wmo_code_map.insert(24, "Freezing drizzle or freezing rain".to_string());
    wmo_code_map.insert(25, "Shower(s) of rain".to_string());
    wmo_code_map.insert(26, "Shower(s) of snow, or of rain and snow".to_string());
    wmo_code_map.insert(27, "Shower(s) of hail*, or of rain and hail".to_string());
    wmo_code_map.insert(28, "Fog or ice fog".to_string());
    wmo_code_map.insert(
        29,
        "Thunderstorm (with or without precipitation)".to_string(),
    );
    wmo_code_map.insert(30, "Slight or moderate duststorm or sandstorm".to_string());
    wmo_code_map.insert(31, "Slight or moderate duststorm or sandstorm".to_string());
    wmo_code_map.insert(32, "Slight or moderate duststorm or sandstorm".to_string());
    wmo_code_map.insert(33, "Severe duststorm or sandstorm".to_string());
    wmo_code_map.insert(34, "Severe duststorm or sandstorm".to_string());
    wmo_code_map.insert(35, "Severe duststorm or sandstorm".to_string());
    wmo_code_map.insert(36, "Slight or moderate blowing snow".to_string());
    wmo_code_map.insert(37, "Heavy drifting snow".to_string());
    wmo_code_map.insert(38, "Slight or moderate blowing snow".to_string());
    wmo_code_map.insert(39, "Heavy drifting snow".to_string());
    wmo_code_map.insert(40, "Fog or ice fog at a distance at the time of observation, but not at the station during the preceding hour, the fog or ice fog extending to a level above that of the observer".to_string());
    wmo_code_map.insert(41, "Fog or ice fog in patches".to_string());
    wmo_code_map.insert(42, "Fog or ice fog, sky visible".to_string());
    wmo_code_map.insert(43, "Fog or ice fog, sky invisible".to_string());
    wmo_code_map.insert(44, "Fog or ice fog, sky visible".to_string());
    wmo_code_map.insert(45, "Fog or ice fog, sky invisible".to_string());
    wmo_code_map.insert(46, "Fog or ice fog, sky visible".to_string());
    wmo_code_map.insert(47, "Fog or ice fog, sky invisible".to_string());
    wmo_code_map.insert(48, "Fog, depositing rime, sky visible".to_string());
    wmo_code_map.insert(49, "Fog, depositing rime, sky invisible".to_string());
    wmo_code_map.insert(50, "Drizzle, not freezing, intermittent".to_string());
    wmo_code_map.insert(51, "Drizzle, not freezing, continuous".to_string());
    wmo_code_map.insert(52, "Drizzle, not freezing, intermittent".to_string());
    wmo_code_map.insert(53, "Drizzle, not freezing, continuous".to_string());
    wmo_code_map.insert(54, "Drizzle, not freezing, intermittent".to_string());
    wmo_code_map.insert(55, "Drizzle, not freezing, continuous".to_string());
    wmo_code_map.insert(56, "Drizzle, freezing, slight".to_string());
    wmo_code_map.insert(
        57,
        "Drizzle, freezing, moderate or heavy (dence)".to_string(),
    );
    wmo_code_map.insert(58, "Drizzle and rain, slight".to_string());
    wmo_code_map.insert(59, "Drizzle and rain, moderate or heavy".to_string());
    wmo_code_map.insert(60, "Rain, not freezing, intermittent".to_string());
    wmo_code_map.insert(61, "Rain, not freezing, continuous".to_string());
    wmo_code_map.insert(62, "Rain, not freezing, intermittent".to_string());
    wmo_code_map.insert(63, "Rain, not freezing, continuous".to_string());
    wmo_code_map.insert(64, "Rain, not freezing, intermittent".to_string());
    wmo_code_map.insert(65, "Rain, not freezing, continuous".to_string());
    wmo_code_map.insert(66, "Rain, freezing, slight".to_string());
    wmo_code_map.insert(67, "Rain, freezing, moderate or heavy (dence)".to_string());
    wmo_code_map.insert(68, "Rain or drizzle and snow, slight".to_string());
    wmo_code_map.insert(
        69,
        "Rain or drizzle and snow, moderate or heavy".to_string(),
    );
    wmo_code_map.insert(70, "Intermittent fall of snowflakes".to_string());
    wmo_code_map.insert(71, "Continuous fall of snowflakes".to_string());
    wmo_code_map.insert(72, "Intermittent fall of snowflakes".to_string());
    wmo_code_map.insert(73, "Continuous fall of snowflakes".to_string());
    wmo_code_map.insert(74, "Intermittent fall of snowflakes".to_string());
    wmo_code_map.insert(75, "Continuous fall of snowflakes".to_string());
    wmo_code_map.insert(76, "Diamond dust (with or without fog)".to_string());
    wmo_code_map.insert(77, "Snow grains (with or without fog)".to_string());
    wmo_code_map.insert(
        78,
        "Isolated star-like snow crystals (with or without fog)".to_string(),
    );
    wmo_code_map.insert(79, "Ice pellets".to_string());
    wmo_code_map.insert(80, "Rain shower(s), slight".to_string());
    wmo_code_map.insert(81, "Rain shower(s), moderate or heavy".to_string());
    wmo_code_map.insert(82, "Rain shower(s), violent".to_string());
    wmo_code_map.insert(83, "Shower(s) of rain and snow mixed, slight".to_string());
    wmo_code_map.insert(
        84,
        "Shower(s) of rain and snow mixed, moderate or heavy".to_string(),
    );
    wmo_code_map.insert(85, "Snow shower(s), slight".to_string());
    wmo_code_map.insert(86, "Snow shower(s), moderate or heavy".to_string());
    wmo_code_map.insert(
        87,
        "Shower(s) of snow pellets or small hail, with or without rain or rain and snow mixed"
            .to_string(),
    );
    wmo_code_map.insert(
        88,
        "Shower(s) of snow pellets or small hail, with or without rain or rain and snow mixed"
            .to_string(),
    );
    wmo_code_map.insert(89, "Shower(s) of hail, with or without rain or rain and snow mixed, not associated with thunder".to_string());
    wmo_code_map.insert(90, "Shower(s) of hail, with or without rain or rain and snow mixed, not associated with thunder".to_string());
    wmo_code_map.insert(91, "Slight rain at time of observation".to_string());
    wmo_code_map.insert(
        92,
        "Moderate or heavy rain at time of observation".to_string(),
    );
    wmo_code_map.insert(
        93,
        "Slight snow, or rain and snow mixed or hail** at time of observation".to_string(),
    );
    wmo_code_map.insert(
        94,
        "Moderate or heavy snow, or rain and snow mixed or hail** at time of observation"
            .to_string(),
    );
    wmo_code_map.insert(
        95,
        "Thunderstorm, slight or moderate, without hail".to_string(),
    );
    wmo_code_map.insert(
        96,
        "Thunderstorm, slight or moderate, with hail".to_string(),
    );
    wmo_code_map.insert(
        97,
        "Thunderstorm, heavy, without hail but with rain and/or snow at time of observation"
            .to_string(),
    );
    wmo_code_map.insert(
        98,
        "Thunderstorm combined with duststorm or sandstorm at time of observation".to_string(),
    );
    wmo_code_map.insert(
        99,
        "Thunderstorm, heavy, with hail at time of observation".to_string(),
    );
    wmo_code_map
}
