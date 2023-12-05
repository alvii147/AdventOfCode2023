use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::{Regex, Split};
use std::fs;

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

fn split_whitespace(s: &str) -> Split {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").expect("Regex should compile");
    }

    return RE.split(s);
}

struct AlmanacMap {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

impl AlmanacMap {
    fn get(&self, k: u64) -> Option<u64> {
        if k >= self.src_start && k < self.src_start + self.length {
            return Some((self.dest_start + k) - self.src_start);
        }

        return None;
    }
}

struct AlmanacConverter {
    maps: Vec<AlmanacMap>,
}

impl AlmanacConverter {
    fn get(&self, k: u64) -> u64 {
        for map in self.maps.iter().map(|x| x) {
            if let Some(v) = map.get(k) {
                return v;
            }
        }

        return k;
    }
}

fn parse_converter(map_str: &str) -> AlmanacConverter {
    let mut maps: Vec<AlmanacMap> = Vec::new();
    for map_line in map_str.split::<char>('\n') {
        let mut nums: Vec<u64> = Vec::new();
        for num_str in split_whitespace(map_line) {
            if let Ok(n) = num_str.parse::<u64>() {
                nums.push(n);
            }
        }

        if nums.len() != 3 {
            panic!("Should parse exactly 3 numbers");
        }

        maps.push(AlmanacMap {
            dest_start: nums[0],
            src_start: nums[1],
            length: nums[2],
        });
    }

    maps.sort_by_key(|c| c.dest_start);
    let converter = AlmanacConverter { maps: maps };

    return converter;
}

fn parse_almanac(
    almanac: &str,
) -> (
    Vec<u64>,
    AlmanacConverter,
    AlmanacConverter,
    AlmanacConverter,
    AlmanacConverter,
    AlmanacConverter,
    AlmanacConverter,
    AlmanacConverter,
) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^\s*seeds:\s*(.+)\n+\s*seed\-to\-soil\s+map:\s*([\S\s]*)\s*soil\-to\-fertilizer\s+map:\s*([\S\s]*)\s*fertilizer\-to\-water\s+map:\s*([\S\s]*)\s*water\-to\-light\s+map:\s*([\S\s]*)\s*light\-to\-temperature\s+map:\s*([\S\s]*)\s*temperature\-to\-humidity\s+map:\s*([\S\s]*)\s*humidity\-to\-location\s+map:\s*([\S\s]*)\s*$"
        ).expect("Regex should compile");
    }

    let caps = RE.captures(almanac).expect("Almanac should be parsed");

    let mut seeds: Vec<u64> = Vec::new();
    for seed_str in split_whitespace(
        caps.get(1)
            .expect("Should be able to capture group")
            .as_str(),
    ) {
        if let Ok(n) = seed_str.parse::<u64>() {
            seeds.push(n);
        }
    }

    let seed2soil: AlmanacConverter = parse_converter(
        caps.get(2)
            .expect("Should be able to capture group")
            .as_str()
            .trim(),
    );
    let soil2fertilizer: AlmanacConverter = parse_converter(
        caps.get(3)
            .expect("Should be able to capture group")
            .as_str()
            .trim(),
    );
    let fertilizer2water: AlmanacConverter = parse_converter(
        caps.get(4)
            .expect("Should be able to capture group")
            .as_str()
            .trim(),
    );
    let water2light: AlmanacConverter = parse_converter(
        caps.get(5)
            .expect("Should be able to capture group")
            .as_str()
            .trim(),
    );
    let light2temperature: AlmanacConverter = parse_converter(
        caps.get(6)
            .expect("Should be able to capture group")
            .as_str()
            .trim(),
    );
    let temperature2humidity: AlmanacConverter = parse_converter(
        caps.get(7)
            .expect("Should be able to capture group")
            .as_str()
            .trim(),
    );
    let humidity2location: AlmanacConverter = parse_converter(
        caps.get(8)
            .expect("Should be able to capture group")
            .as_str()
            .trim(),
    );

    return (
        seeds,
        seed2soil,
        soil2fertilizer,
        fertilizer2water,
        water2light,
        light2temperature,
        temperature2humidity,
        humidity2location,
    );
}

fn main() {
    let filename: &str = "../almanac.txt";
    let file_contents: String = read_file(filename);
    let (
        seeds,
        seed2soil,
        soil2fertilizer,
        fertilizer2water,
        water2light,
        light2temperature,
        temperature2humidity,
        humidity2location,
    ) = parse_almanac(&file_contents);

    let mut lowest_location = u64::MAX;
    for &seed in seeds.iter().map(|x| x) {
        let soil = seed2soil.get(seed);
        let fertilizer = soil2fertilizer.get(soil);
        let water = fertilizer2water.get(fertilizer);
        let light = water2light.get(water);
        let temperature = light2temperature.get(light);
        let humidity = temperature2humidity.get(temperature);
        let location = humidity2location.get(humidity);
        lowest_location = u64::min(lowest_location, location);
    }

    println!("{:?}", lowest_location);

    let lowests_in_range = seeds
        .par_chunks(2)
        .map(|seed_pair| {
            let mut lowest_in_range = u64::MAX;
            for seed in seed_pair[0]..(seed_pair[0] + seed_pair[1]) {
                let soil = seed2soil.get(seed);
                let fertilizer = soil2fertilizer.get(soil);
                let water = fertilizer2water.get(fertilizer);
                let light = water2light.get(water);
                let temperature = light2temperature.get(light);
                let humidity = temperature2humidity.get(temperature);
                let location = humidity2location.get(humidity);

                lowest_in_range = u64::min(lowest_in_range, location);
            }

            return lowest_in_range;
        })
        .collect::<Vec<u64>>();

    lowest_location = u64::MAX;
    for l in lowests_in_range {
        lowest_location = u64::min(lowest_location, l);
    }

    println!("{:?}", lowest_location);
}
