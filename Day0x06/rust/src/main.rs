use std::{fs, iter::zip};

use lazy_static::lazy_static;
use regex::Regex;

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

fn quadratic(a: i64, b: i64, c: i64) -> Vec<f64> {
    let mut solutions: Vec<f64> = Vec::new();
    let discriminant: i64 = i64::pow(b, 2) - (4 * a * c);
    if discriminant < 0 {
        return solutions;
    }

    let minus_b: f64 = -(b as f64);
    let two_a: f64 = (2 * a) as f64;
    let sqrt_discriminant: f64 = f64::sqrt(discriminant as f64);

    solutions.push((minus_b + sqrt_discriminant) / two_a);
    if discriminant == 0 {
        return solutions;
    }

    solutions.push((minus_b - sqrt_discriminant) / two_a);

    return solutions;
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_races(s: &str) -> Vec<Race> {
    lazy_static! {
        static ref RE_TIME: Regex = Regex::new(r"Time:([\s\d]+)").expect("Regex should compile");
        static ref RE_DISTANCE: Regex =
            Regex::new(r"Distance:([\s\d]+)").expect("Regex should compile");
    }

    let times: Vec<u64> = RE_TIME
        .captures(s)
        .expect("Should be able to capture groups")
        .get(1)
        .expect("Should be able to parse times")
        .as_str()
        .split_whitespace()
        .map(|t| t.parse::<u64>().expect("Time should be parsed"))
        .collect::<Vec<u64>>();

    let distances: Vec<u64> = RE_DISTANCE
        .captures(s)
        .expect("Should be able to capture groups")
        .get(1)
        .expect("Should be able to parse times")
        .as_str()
        .split_whitespace()
        .map(|d| d.parse::<u64>().expect("Time should be parsed"))
        .collect::<Vec<u64>>();

    let races: Vec<Race> = zip(times, distances)
        .map(|(t, d)| Race {
            time: t,
            distance: d,
        })
        .collect::<Vec<Race>>();

    return races;
}

fn parse_race(s: &str) -> Race {
    lazy_static! {
        static ref RE_TIME: Regex = Regex::new(r"Time:([\s\d]+)").expect("Regex should compile");
        static ref RE_DISTANCE: Regex =
            Regex::new(r"Distance:([\s\d]+)").expect("Regex should compile");
    }

    let time: u64 = RE_TIME
        .captures(s)
        .expect("Should be able to capture groups")
        .get(1)
        .expect("Should be able to parse times")
        .as_str()
        .split_whitespace()
        .fold("".to_string(), |acc: String, t: &str| acc + t)
        .parse::<u64>()
        .expect("Time should be parsed");

    let distance: u64 = RE_DISTANCE
        .captures(s)
        .expect("Should be able to capture groups")
        .get(1)
        .expect("Should be able to parse times")
        .as_str()
        .split_whitespace()
        .fold("".to_string(), |acc: String, d: &str| acc + d)
        .parse::<u64>()
        .expect("Time should be parsed");

    let race: Race = Race {
        time: time,
        distance: distance,
    };

    return race;
}

fn round_next_integer(f: f64, up: bool) -> i64 {
    if f.fract() == 0.0 {
        let i = f as i64;
        if up {
            return i + 1;
        }

        return i - 1;
    }

    if up {
        return f.ceil() as i64;
    }

    return f.floor() as i64;
}

fn compute_num_solutions(race: Race) -> i64 {
    let solutions = quadratic(1, -(race.time as i64), race.distance as i64);
    if solutions.len() != 2 {
        panic!("Expected 2 solutions");
    }

    let lowest_solution: f64 = solutions.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let highest_solution: f64 = solutions.iter().fold(0 as f64, |a, &b| a.max(b));
    let lowest_int_solution: i64 = round_next_integer(lowest_solution, true);
    let highest_int_solution: i64 = round_next_integer(highest_solution, false);

    let num_solutions: i64 = highest_int_solution - lowest_int_solution + 1;

    return num_solutions;
}

fn main() {
    let filename: &str = "../races.txt";
    let file_contents: String = read_file(filename);
    let races: Vec<Race> = parse_races(&file_contents);
    let mut prod_solutions: i64 = 1;
    for race in races {
        prod_solutions *= compute_num_solutions(race);
    }

    println!("{:?}", prod_solutions);

    let race = parse_race(&file_contents);
    let num_solutions: i64 = compute_num_solutions(race);

    println!("{:?}", num_solutions);
}
