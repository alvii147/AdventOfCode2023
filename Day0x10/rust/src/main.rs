use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use lazy_static::lazy_static;

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    Ground,
}

lazy_static! {
    static ref CHAR_PIPE_MAP: HashMap<char, Pipe> = HashMap::from([
        ('|', Pipe::Vertical),
        ('-', Pipe::Horizontal),
        ('L', Pipe::NorthEast),
        ('J', Pipe::NorthWest),
        ('F', Pipe::SouthEast),
        ('7', Pipe::SouthWest),
        ('S', Pipe::Start),
        ('.', Pipe::Ground),
    ]);
    static ref PIPE_DIRECTION_SET_MAP: HashMap<Pipe, HashSet<Direction>> = HashMap::from([
        (
            Pipe::Vertical,
            HashSet::from([Direction::Top, Direction::Bottom])
        ),
        (
            Pipe::Horizontal,
            HashSet::from([Direction::Left, Direction::Right])
        ),
        (
            Pipe::NorthEast,
            HashSet::from([Direction::Top, Direction::Right])
        ),
        (
            Pipe::NorthWest,
            HashSet::from([Direction::Top, Direction::Left])
        ),
        (
            Pipe::SouthEast,
            HashSet::from([Direction::Bottom, Direction::Right])
        ),
        (
            Pipe::SouthWest,
            HashSet::from([Direction::Bottom, Direction::Left])
        ),
        (
            Pipe::Start,
            HashSet::from([
                Direction::Top,
                Direction::Bottom,
                Direction::Left,
                Direction::Right
            ])
        ),
        (Pipe::Ground, HashSet::new()),
    ]);
}

type Coordinate = (i64, i64);

#[derive(Debug)]
struct CoordinateQueueElement {
    coordinates: Coordinate,
    distance: u64,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    pipe: Pipe,
    shortest_distance: Option<u64>,
    visited: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RayCastingState {
    None,
    Up,
    Down,
}

fn main() {
    let filename: &str = "../tiles.txt";
    let file_contents: String = read_file(filename);
    let lines: Vec<&str> = file_contents.split::<char>('\n').collect::<Vec<&str>>();

    let n_rows: usize = lines.len();
    let mut n_cols: usize = 0;

    let mut coor_queue: VecDeque<CoordinateQueueElement> = VecDeque::new();
    let mut tiles_map: HashMap<Coordinate, Tile> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        n_cols = line.len();
        for (j, p) in line.chars().enumerate() {
            let pipe: Pipe = *CHAR_PIPE_MAP.get(&p).expect("Should be able to parse pipe");
            if pipe == Pipe::Start {
                coor_queue.push_back(CoordinateQueueElement {
                    coordinates: (i as i64, j as i64),
                    distance: 0,
                });
            }

            tiles_map.insert(
                (i as i64, j as i64),
                Tile {
                    pipe: pipe,
                    shortest_distance: None,
                    visited: false,
                },
            );
        }
    }

    while coor_queue.len() > 0 {
        let coor_queue_elem: CoordinateQueueElement = coor_queue.pop_front().unwrap();
        let coor: Coordinate = coor_queue_elem.coordinates;
        let tile: Tile = *tiles_map.get(&coor).unwrap();
        let tile_pipe_dir: &HashSet<Direction> = PIPE_DIRECTION_SET_MAP.get(&tile.pipe).unwrap();

        let mut new_distance: u64 = coor_queue_elem.distance;
        if let Some(current_distance) = tile.shortest_distance {
            new_distance = u64::min(new_distance, current_distance);
        }

        let mut going_up: bool = false;
        let top_coor: Coordinate = (coor.0 - 1, coor.1);
        if let Some(&top_tile) = tiles_map.get(&top_coor) {
            if !top_tile.visited
                && tile_pipe_dir.contains(&Direction::Top)
                && PIPE_DIRECTION_SET_MAP
                    .get(&top_tile.pipe)
                    .unwrap()
                    .contains(&Direction::Bottom)
            {
                coor_queue.push_back(CoordinateQueueElement {
                    coordinates: top_coor,
                    distance: coor_queue_elem.distance + 1,
                });
                going_up = true;
            }
        }

        let mut going_down: bool = false;
        let bottom_coor: Coordinate = (coor.0 + 1, coor.1);
        if let Some(&bottom_tile) = tiles_map.get(&bottom_coor) {
            if !bottom_tile.visited
                && tile_pipe_dir.contains(&Direction::Bottom)
                && PIPE_DIRECTION_SET_MAP
                    .get(&bottom_tile.pipe)
                    .unwrap()
                    .contains(&Direction::Top)
            {
                coor_queue.push_back(CoordinateQueueElement {
                    coordinates: bottom_coor,
                    distance: coor_queue_elem.distance + 1,
                });
                going_down = true;
            }
        }

        let mut going_left: bool = false;
        let left_coor: Coordinate = (coor.0, coor.1 - 1);
        if let Some(&left_tile) = tiles_map.get(&left_coor) {
            if !left_tile.visited
                && tile_pipe_dir.contains(&Direction::Left)
                && PIPE_DIRECTION_SET_MAP
                    .get(&left_tile.pipe)
                    .unwrap()
                    .contains(&Direction::Right)
            {
                coor_queue.push_back(CoordinateQueueElement {
                    coordinates: left_coor,
                    distance: coor_queue_elem.distance + 1,
                });
                going_left = true;
            }
        }

        let mut going_right: bool = false;
        let right_coor: Coordinate = (coor.0, coor.1 + 1);
        if let Some(&right_tile) = tiles_map.get(&right_coor) {
            if !right_tile.visited
                && tile_pipe_dir.contains(&Direction::Right)
                && PIPE_DIRECTION_SET_MAP
                    .get(&right_tile.pipe)
                    .unwrap()
                    .contains(&Direction::Left)
            {
                coor_queue.push_back(CoordinateQueueElement {
                    coordinates: right_coor,
                    distance: coor_queue_elem.distance + 1,
                });
                going_right = true;
            }
        }

        let mut pipe: Pipe = tile.pipe;
        if tile.pipe == Pipe::Start {
            if going_up && going_down {
                pipe = Pipe::Vertical;
            } else if going_up && going_left {
                pipe = Pipe::NorthWest;
            } else if going_up && going_right {
                pipe = Pipe::NorthEast;
            } else if going_down && going_left {
                pipe = Pipe::SouthWest;
            } else if going_down && going_left {
                pipe = Pipe::SouthEast;
            } else if going_left && going_right {
                pipe = Pipe::Horizontal;
            } else {
                panic!("Can't determine starting pipe");
            }
        }

        tiles_map.insert(
            coor,
            Tile {
                pipe: pipe,
                visited: true,
                shortest_distance: Some(new_distance),
            },
        );
    }

    let mut longest_distance: u64 = u64::MIN;
    for (_, &tile) in tiles_map.iter() {
        if let Some(shortest_distance) = tile.shortest_distance {
            longest_distance = u64::max(longest_distance, shortest_distance);
        }
    }

    println!("{:?}", longest_distance);

    let mut n_enclosed: u64 = 0;

    for i in 0..n_rows {
        let mut ray_casting_state: RayCastingState = RayCastingState::None;
        let mut edge_state: i8 = -1;

        for j in 0..n_cols {
            if let Some(t) = tiles_map.get(&(i as i64, j as i64)) {
                let is_main_loop: bool = t.shortest_distance != None;

                match ray_casting_state {
                    RayCastingState::None => {
                        if is_main_loop {
                            match t.pipe {
                                Pipe::Vertical => edge_state *= -1,
                                Pipe::NorthEast => ray_casting_state = RayCastingState::Up,
                                Pipe::SouthEast => ray_casting_state = RayCastingState::Down,
                                Pipe::Horizontal
                                | Pipe::NorthWest
                                | Pipe::SouthWest
                                | Pipe::Ground
                                | Pipe::Start => panic!("Unexpected pipe {:?}", t.pipe),
                            }
                        } else {
                            if edge_state == 1 {
                                n_enclosed += 1;
                            }
                        }
                    }
                    RayCastingState::Up => {
                        if is_main_loop {
                            match t.pipe {
                                Pipe::Horizontal => (),
                                Pipe::NorthWest => ray_casting_state = RayCastingState::None,
                                Pipe::SouthWest => {
                                    ray_casting_state = RayCastingState::None;
                                    edge_state *= -1;
                                }
                                Pipe::Vertical
                                | Pipe::NorthEast
                                | Pipe::SouthEast
                                | Pipe::Ground
                                | Pipe::Start => panic!("Unexpected pipe {:?}", t.pipe),
                            }
                        } else {
                            panic!("Expected part of loop");
                        }
                    }
                    RayCastingState::Down => {
                        if is_main_loop {
                            match t.pipe {
                                Pipe::Horizontal => (),
                                Pipe::NorthWest => {
                                    ray_casting_state = RayCastingState::None;
                                    edge_state *= -1;
                                }
                                Pipe::SouthWest => ray_casting_state = RayCastingState::None,
                                Pipe::Vertical
                                | Pipe::NorthEast
                                | Pipe::SouthEast
                                | Pipe::Ground
                                | Pipe::Start => panic!(),
                            }
                        } else {
                            panic!("Expected part of loop");
                        }
                    }
                }
            }
        }
    }

    println!("{}", n_enclosed);
}
