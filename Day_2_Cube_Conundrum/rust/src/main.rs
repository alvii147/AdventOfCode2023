use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::max, collections::HashMap, fs, str::Split};

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

fn game_possibility(line: &str, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
    lazy_static! {
        static ref RE_LINE: Regex =
            Regex::new(r"^Game (\d+):\s*(.*)\s*$").expect("Should be able to parse line");
        static ref RE_CUBE: Regex = Regex::new(r"^\s*(\d+)\s+(\S+)\s*$")
            .expect("Should be able to parse cube count and color");
    }

    let max_count_map: HashMap<&str, u32> =
        HashMap::from([(RED, max_red), (GREEN, max_green), (BLUE, max_blue)]);

    let line_caps = RE_LINE
        .captures(line)
        .expect("Should be able to parse line");
    let game_id: u32 = line_caps
        .get(1)
        .expect("Should be able to parse game ID")
        .as_str()
        .parse::<u32>()
        .expect("Game ID should be positive integer");
    let cubes_str: &str = line_caps
        .get(2)
        .expect("Should be able to parse cubes data")
        .as_str();

    let draw_strs: Split<char> = cubes_str.split::<char>(';');
    for draw_str in draw_strs {
        let cube_strs: Split<char> = draw_str.split::<char>(',');
        for cube_str in cube_strs {
            let cube_caps = RE_CUBE
                .captures(cube_str)
                .expect("Should be able to parse cube data");
            let cube_count: u32 = cube_caps
                .get(1)
                .expect("Should be able to cube count")
                .as_str()
                .parse::<u32>()
                .expect("Cube count should be positive integer");
            let cube_color: &str = cube_caps
                .get(2)
                .expect("Should be able to parse cube color")
                .as_str();

            let &max_cube_count: &u32 = max_count_map
                .get(cube_color)
                .expect("Should be able to look up cube color");
            if cube_count > max_cube_count {
                return 0;
            }
        }
    }

    return game_id;
}

fn game_power(line: &str) -> u32 {
    lazy_static! {
        static ref RE_LINE: Regex =
            Regex::new(r"^Game \d+:\s*(.*)\s*$").expect("Should be able to parse line");
        static ref RE_CUBE: Regex = Regex::new(r"^\s*(\d+)\s+(\S+)\s*$")
            .expect("Should be able to parse cube count and color");
    }

    let mut fewest_counts_map: HashMap<&str, u32> =
        HashMap::from([(RED, 0), (GREEN, 0), (BLUE, 0)]);

    let line_caps = RE_LINE
        .captures(line)
        .expect("Should be able to parse line");
    let cubes_str: &str = line_caps
        .get(1)
        .expect("Should be able to parse cubes data")
        .as_str();

    let draw_strs: Split<char> = cubes_str.split::<char>(';');
    for draw_str in draw_strs {
        let cube_strs: Split<char> = draw_str.split::<char>(',');
        for cube_str in cube_strs {
            let cube_caps = RE_CUBE
                .captures(cube_str)
                .expect("Should be able to parse cube data");
            let cube_count: u32 = cube_caps
                .get(1)
                .expect("Should be able to cube count")
                .as_str()
                .parse::<u32>()
                .expect("Cube count should be positive integer");
            let cube_color: &str = cube_caps
                .get(2)
                .expect("Should be able to parse cube color")
                .as_str();

            let &fewest_cube_count: &u32 = fewest_counts_map
                .get(cube_color)
                .expect("Should be able to look up cube color");
            fewest_counts_map.insert(cube_color, max(cube_count, fewest_cube_count));
            // if cube_count > fewest_cube_count {
            //     fewest_counts_map.insert(cube_color, cube_count);
            // }
        }
    }

    let &fewest_red_count: &u32 = fewest_counts_map
        .get(RED)
        .expect("Should be able to get red count");
    let &fewest_green_count: &u32 = fewest_counts_map
        .get(GREEN)
        .expect("Should be able to get red count");
    let &fewest_blue_count: &u32 = fewest_counts_map
        .get(BLUE)
        .expect("Should be able to get red count");

    return fewest_red_count * fewest_green_count * fewest_blue_count;
}

fn main() {
    let filename: &str = "../games.txt";
    let file_contents: String = read_file(filename);
    let lines: Split<char> = file_contents.split::<char>('\n');

    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    let mut sum_ids: u32 = 0;
    let mut sum_powers: u32 = 0;

    for line in lines {
        sum_ids += game_possibility(line, max_red, max_green, max_blue);
        sum_powers += game_power(line);
    }

    println!("{}", sum_ids);
    println!("{}", sum_powers);
}
