# Salati

Salati is a Rust that calculate prayer times according to most recent scientific research.

>  **DISCLAIMER** The code in this repo, is mainly copied from [salah](https://github.com/insha/salah). Typially, I'd create a PR in the other repositoy, but as I'm doing many changes where some are controversial I decided to create a new repository.

## Why

My main motivations to create a new repository are the followings:

- Web Assembly support: planning to integrate it to [muslim-box](https://github.com/benzid-wael/muslim-box)
- Surface assumptions/adjustments to users: One of my main concerns when using 3rd-party apps/libraries is being unaware of assumptions/corrections/adjustments which put me in ambiguity regarding prayer times. Note that it's we depend on this information to know imsak, iftar and prayer end time as well.
- Have more control over prayer time calculation:
   - Which twilight to use to compute Isha time?
   - How to calculate prayer time for places in polar circle?


## Usage

Add the following to your Cargo.toml file under the 

```
[dependencies]
salah = "0.0.0"
```

To get prayer times, do the following

```rust
use salati::prelude::*;

const tunis = Coordinates::new(36.8065, 10.1815);
const date  = Utc.ymd(2022, 8, 1);
const params = ParametersBuilder::with(Method::Tunisia, Madhab::Shafi);
const prayers = PrayerTimes::new(date, coordinates, params);

println!("fajr: {}", prayers.fajr.datetime.unwrap());
println!("sunrise: {}", prayers.sunrise.datetime.unwrap());
println!("dhuhr: {}", prayers.dhuhr.datetime.unwrap());
println!("asr: {}", prayers.asr.datetime.unwrap());
println!("maghrib: {}", prayers.maghrib.datetime.unwrap());
println!("isha: {}", prayers.isha.datetime.         unwrap());
```

## Configuration

You can configure your prayer times calculater as follow:

| Parameter              | Description |
| ---------------------- | ----------- |
| `method`               | used to populate configurations used by known Muslim organizations |
| `fajr_angle`           | Angle of the sun used to calculate Fajr |
| `isha_angle`           | Angle of the sun used to calculate Isha |
| `isha_interval`        | Minutes after Maghrib (if set, the time for Isha will be Maghrib plus `isha_interval`) |
| `madhab`               | used to calculate Asr time  |
| `twilight`             | used to calculate Isha time |
| `high_latitude_rule`   | used to set a minimum time for Fajr and a max time for Isha |
| `method_adjustments`   | method time adjustment |
| `adjustments`          | custom prayer time adjustments in minutes for each prayer time. By default, all values are `0`.|

### Method

Provides preset configuration for a few authorities for calculating prayer times.

| Value | Description |
| ----- | ----------- |
| `MuslimWorldLeague` | Muslim World League. Fajr angle: 18, Isha angle: 17 |
| `Egyptian` | Egyptian General Authority of Survey. Fajr angle: 19.5, Isha angle: 17.5 |
| `Karachi` | University of Islamic Sciences, Karachi. Fajr angle: 18, Isha angle: 18 |
| `UmmAlQura` | Umm al-Qura University, Makkah. Fajr angle: 18.5, Isha interval: 90. *Note*: you should add a *+30 minute* custom adjustment for Isha during *Ramadan*. |
| `Dubai` | Method used in UAE. Fajr angle: 18.2, Isha angle: 18.2. |
| `Qatar` | Modified version of Umm al-Qura used in Qatar. Fajr angle: 18, Isha interval: 90. |
| `Kuwait` | Method used by the country of Kuwait. Fajr angle: 18, Isha angle: 17.5 |
| `MoonsightingCommittee` | Moonsighting Committee. Fajr angle: 18, Isha angle: 18. Also uses seasonal adjustment values. |
| `Singapore` | Method used by Singapore. Fajr angle: 20, Isha angle: 18. |
| `NorthAmerica` | Referred to as the ISNA method. Fajr angle: 15, Isha angle: 15 |
| `Other` | Fajr angle: 0, Isha angle: 0. This is the default value for when manually initializing the `Parameters` struct. |


### Madhab

This setting is used only to calulate Asr prayer time:

| Value    | Description |
| -------- | ----------- |
| `Shafi`  | Will result in an earlier Asr time (default) |
| `Hanafi` | Will result in a later Asr time |

### Twilight 🔜

Used to calculate Isha prayer time:

| Value   | Description |
| ------- | ----------- |
| `Red`   | Earlier Asr time (default) |
| `White` | Later Asr time |


### High Latitude Rule

Rule to adjust Fajr/Isha prayer time, we will fallback to this method when we end with invalid prayer times 🔜

| Value | Description |
| ----- | ----------- |
| `MiddleOfTheNight` | Fajr will never be earlier than the middle of the night and Isha will never be later than the middle of the night |
| `SeventhOfTheNight` | Fajr will never be earlier than the beginning of the last seventh of the night and Isha will never be later than the end of the first seventh of the night |
| `TwilightAngle` | Similar to SeventhOfTheNight, but instead of 1/7, the fraction of the night used is fajr_angle/60 and isha_angle/60 (default) |


You are not sure which strategy to use? use `recommended` method:

```rust
const high_latitude_resolution = HighLatitudeRule::recommended(coordinates);
```

### Polar Circle Resolution 🔜

Used to resolve undefined prayer time in areas located in the polar circle whe sun does not set/rise.

| Value | Description |
| ----- | ----------- |
| `NearestPlace` | Finds the closest location for which sunrise and sunset prayer times can be computed |
| `NearestDay`   | Finds the closest date (forward or backward) for which sunrise and sunset prayer times can be computed |
| `Unresolved`   | (default) Leaves all prayer undefined |
| `UmmAlQura`    | Calculate prayer times based on Umm Al-Qura location |


## Development

1. Clone Repository
1. Set-up `pre-commit`
```shell
pipenv install
pipenv run pre-commit install
```
1. Make your changes into a new new branch
1. Add unit tests for your changes
1. Run unit tests: `cargo test`

