use serde::{Deserialize, Serialize};
use time::{macros::format_description, Date, Month};
use wasm_bindgen::prelude::*;

const HOT_TEMPERATURE: isize = 34;
const COLD_TEMPERATURE: isize = 10;
const TEMPERATURE_RISE_THRESHOLD: isize = 5;
const TEMPERATURE_DROP_THRESHOLD: isize = -5;
const FEEL_TEMPERATURE_RISE_THRESHOLD: isize = 5;
const FEEL_TEMPERATURE_DROP_THRESHOLD: isize = -5;
const TEMPERATURE_STABLE_RANGE: isize = 5;
const EXTREME_TEMPERATURE_CHANGE: isize = 10;
const COMFORTABLE_HIGH_TEMP: isize = 30;
const COMFORTABLE_LOW_TEMP: isize = 20;
const WIND_CHILL_DIFFERENCE: isize = -5;
const HEAT_INDEX_DIFFERENCE: isize = 5;

struct TempRanges {
    high: isize,
    low: isize,
}

impl From<Date> for TempRanges {
    fn from(value: Date) -> Self {
        match value.month() {
            Month::January => Self { high: 19, low: 12 },
            Month::February => Self { high: 20, low: 13 },
            Month::March => Self { high: 23, low: 16 },
            Month::April => Self { high: 26, low: 19 },
            Month::May => Self { high: 29, low: 22 },
            Month::June => Self { high: 31, low: 24 },
            Month::July => Self { high: 33, low: 26 },
            Month::August => Self { high: 32, low: 26 },
            Month::September => Self { high: 31, low: 24 },
            Month::October => Self { high: 28, low: 22 },
            Month::November => Self { high: 24, low: 18 },
            Month::December => Self { high: 20, low: 14 },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct WeatherInfo {
    high: isize,
    low: isize,
    feel: isize,
    date: String,
}

#[wasm_bindgen(js_name = "describeWeatherChange")]
pub fn describe_weather_change(today: JsValue, tomorrow: JsValue) -> Result<String, JsError> {
    let today = serde_wasm_bindgen::from_value::<WeatherInfo>(today)?;
    let tomorrow = serde_wasm_bindgen::from_value::<WeatherInfo>(tomorrow)?;

    let format = format_description!("[year]-[month]-[day]");
    let today_range = TempRanges::from(Date::parse(&today.date, format)?);
    let tomorrow_range = TempRanges::from(Date::parse(&tomorrow.date, format)?);

    #[rustfmt::skip]
    let result = match (today, tomorrow) {
        (today, tomorrow) if (tomorrow.high - today.high) > TEMPERATURE_RISE_THRESHOLD => "天氣變熱",
        (today, tomorrow) if tomorrow.high > HOT_TEMPERATURE || today.high > HOT_TEMPERATURE => "非常炎熱",
        (today, tomorrow) if tomorrow.high < COLD_TEMPERATURE || today.high < COLD_TEMPERATURE => "非常寒冷",
        (today, tomorrow) if (tomorrow.low - today.low) > TEMPERATURE_RISE_THRESHOLD => "夜晚變暖",
        (today, tomorrow) if (tomorrow.feel - today.feel) > FEEL_TEMPERATURE_RISE_THRESHOLD => "體感變熱",
        (today, tomorrow) if (tomorrow.feel - today.feel) < FEEL_TEMPERATURE_DROP_THRESHOLD => "體感變冷",
        (today, tomorrow) if (tomorrow.feel - today.feel) < TEMPERATURE_DROP_THRESHOLD => "天氣變冷",
        (today, tomorrow) if (tomorrow.high - tomorrow.low).abs() - (tomorrow.high - today.low).abs() > TEMPERATURE_STABLE_RANGE => "溫差變大",
        (today, tomorrow) if (tomorrow.high - tomorrow.low).abs() - (tomorrow.high - today.low).abs() < -TEMPERATURE_STABLE_RANGE => "溫差變小",
        (_today, tomorrow) if (tomorrow.feel - tomorrow.high).abs() > HEAT_INDEX_DIFFERENCE => "體感溫度差變大",
        (_today, tomorrow) if (tomorrow.feel - tomorrow.high).abs() > HEAT_INDEX_DIFFERENCE => "體感溫度差變小",
        (today, tomorrow) if today.feel > COMFORTABLE_HIGH_TEMP || tomorrow.feel > COMFORTABLE_HIGH_TEMP => "體感變炎熱",
        (today, tomorrow) if today.feel < COMFORTABLE_LOW_TEMP || tomorrow.feel < COMFORTABLE_LOW_TEMP => "體感變涼爽",
        (today, tomorrow) if today.feel - today.high > HEAT_INDEX_DIFFERENCE || tomorrow.feel - tomorrow.high > HEAT_INDEX_DIFFERENCE => "體感變悶熱",
        (today, tomorrow) if today.feel - today.high < WIND_CHILL_DIFFERENCE || tomorrow.feel - tomorrow.high < WIND_CHILL_DIFFERENCE => "體感變風寒",
        (_today, tomorrow) if (tomorrow.high - tomorrow.low).abs() < TEMPERATURE_STABLE_RANGE => "溫度變化平穩",
        (_today, tomorrow) if (tomorrow.high - tomorrow.low).abs() < EXTREME_TEMPERATURE_CHANGE => "溫度變化劇烈",
        (today, tomorrow) if today.high < today_range.high && tomorrow.high > tomorrow_range.high + TEMPERATURE_RISE_THRESHOLD => "氣溫驟升",
        (today, tomorrow) if today.low > today_range.low && tomorrow.low < tomorrow_range.low - TEMPERATURE_DROP_THRESHOLD => "氣溫驟降",
        _ => "天氣變化不大"
    }.to_owned();

    Ok(result)
}
