use std::{ops::Range, str::FromStr};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

pub fn part1(input: &str) -> Result<String> {
    let maps: Maps = input.parse()?;
    let result = (0..maps.seeds.len())
        .fold(None, |acc, i| {
            let current = MapsEnum::iter().fold(maps.seeds[i], |acc, val| maps.get(val).map(acc));
            if acc.is_none() || acc.is_some_and(|v| current < v) {
                Some(current)
            } else {
                acc
            }
        })
        .map(|v| v.to_string())
        .unwrap_or("Error, no destination found at all".to_string());

    Ok(result)
}

pub fn part2(input: &str) -> Result<String> {
    let maps: Maps = input.parse()?;
    let mut lowest_dest_location = None;

    for range in maps.seeds.iter().chunks(2).into_iter().map(|mut val| {
        let start = *val.next().unwrap();
        Range {
            start,
            end: start + *val.next().unwrap(),
        }
    }) {
        for seed in range.start..range.end {
            let mut current = seed;
            for t in MapsEnum::iter() {
                current = maps.get(t).map(current);
            }
            if lowest_dest_location.is_none() || lowest_dest_location.is_some_and(|v| current < v) {
                lowest_dest_location = Some(current);
            }
        }
    }

    Ok(lowest_dest_location.unwrap().to_string())
}

#[derive(Debug, EnumIter)]
enum MapsEnum {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl FromStr for MapsEnum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed-to-soil map:" => Ok(MapsEnum::SeedToSoil),
            "soil-to-fertilizer map:" => Ok(MapsEnum::SoilToFertilizer),
            "fertilizer-to-water map:" => Ok(MapsEnum::FertilizerToWater),
            "water-to-light map:" => Ok(MapsEnum::WaterToLight),
            "light-to-temperature map:" => Ok(MapsEnum::LightToTemperature),
            "temperature-to-humidity map:" => Ok(MapsEnum::TemperatureToHumidity),
            "humidity-to-location map:" => Ok(MapsEnum::HumidityToLocation),
            _ => bail!("Invalid map"),
        }
    }
}

#[derive(Debug, Default)]
struct Maps {
    seeds: Vec<i64>,
    seed_to_soil: SeedMaps,
    soil_to_fertilizer: SeedMaps,
    fertilizer_to_water: SeedMaps,
    water_to_light: SeedMaps,
    light_to_temperature: SeedMaps,
    temperature_to_humidity: SeedMaps,
    humidity_to_location: SeedMaps,
}

impl Maps {
    fn assign(&mut self, map: MapsEnum, seed_map: Vec<SeedMap>) {
        match map {
            MapsEnum::SeedToSoil => self.seed_to_soil = SeedMaps(seed_map),
            MapsEnum::SoilToFertilizer => self.soil_to_fertilizer = SeedMaps(seed_map),
            MapsEnum::FertilizerToWater => self.fertilizer_to_water = SeedMaps(seed_map),
            MapsEnum::WaterToLight => self.water_to_light = SeedMaps(seed_map),
            MapsEnum::LightToTemperature => self.light_to_temperature = SeedMaps(seed_map),
            MapsEnum::TemperatureToHumidity => self.temperature_to_humidity = SeedMaps(seed_map),
            MapsEnum::HumidityToLocation => self.humidity_to_location = SeedMaps(seed_map),
        }
    }

    fn get(&self, map: MapsEnum) -> &SeedMaps {
        match map {
            MapsEnum::SeedToSoil => &self.seed_to_soil,
            MapsEnum::SoilToFertilizer => &self.soil_to_fertilizer,
            MapsEnum::FertilizerToWater => &self.fertilizer_to_water,
            MapsEnum::WaterToLight => &self.water_to_light,
            MapsEnum::LightToTemperature => &self.light_to_temperature,
            MapsEnum::TemperatureToHumidity => &self.temperature_to_humidity,
            MapsEnum::HumidityToLocation => &self.humidity_to_location,
        }
    }
}

impl FromStr for Maps {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps = Maps::default();

        let mut lines = s.split("\n\n").filter(|line| !line.is_empty());
        maps.seeds = lines
            .next()
            .context("No seeds line present")?
            .split_once(": ")
            .context("Invalid seeds line")?
            .1
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();

        for seed_map in lines {
            let mut lines = seed_map.lines();
            let seed_map_type: MapsEnum = lines.next().unwrap().parse().unwrap();
            let mappings = lines
                .map(|line| {
                    let mut nums = line.split_whitespace();
                    let dest_start: i64 = nums.next().unwrap().parse().unwrap();
                    let source_start: i64 = nums.next().unwrap().parse().unwrap();
                    let length: i64 = nums.next().unwrap().parse().unwrap();
                    SeedMap {
                        source_range: source_start..source_start + length,
                        destination_range: dest_start..dest_start + length,
                    }
                })
                .collect_vec();

            maps.assign(seed_map_type, mappings);
        }

        Ok(maps)
    }
}

#[derive(Debug, Default)]
struct SeedMaps(Vec<SeedMap>);

impl SeedMaps {
    fn map(&self, source: i64) -> i64 {
        self.0.iter().find_map(|map| map.map(source)).unwrap_or(source)
    }
}

#[derive(Debug, Default)]
struct SeedMap {
    source_range: Range<i64>,
    destination_range: Range<i64>,
}

impl SeedMap {
    fn map(&self, source: i64) -> Option<i64> {
        if self.source_range.contains(&source) {
            let offset = source - self.source_range.start;
            Some(self.destination_range.start + offset)
        } else {
            None
        }
    }
}
