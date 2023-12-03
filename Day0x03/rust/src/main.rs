use std::{collections::HashMap, fs, str::Split};

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

struct EngineNumber {
    row_idx: usize,
    start_col_idx: usize,
    end_col_idx: usize,
    num: u32,
}

struct EngineGear {
    n_part_nums: usize,
    ratio: u32,
}

impl EngineNumber {
    fn surrounding_indices(&self, n_rows: usize, n_cols: usize) -> Vec<(usize, usize)> {
        let mut indices: Vec<(usize, usize)> = Vec::new();
        if self.start_col_idx > 0 {
            indices.push((self.row_idx, self.start_col_idx - 1));
        }

        if self.end_col_idx < n_cols - 1 {
            indices.push((self.row_idx, self.end_col_idx));
        }

        if self.row_idx > 0 {
            if self.start_col_idx > 0 {
                indices.push((self.row_idx - 1, self.start_col_idx - 1));
            }

            if self.end_col_idx < n_cols - 1 {
                indices.push((self.row_idx - 1, self.end_col_idx));
            }

            for i in self.start_col_idx..self.end_col_idx {
                indices.push((self.row_idx - 1, i));
            }
        }

        if self.row_idx < n_rows - 1 {
            if self.start_col_idx > 0 {
                indices.push((self.row_idx + 1, self.start_col_idx - 1));
            }

            if self.end_col_idx < n_cols - 1 {
                indices.push((self.row_idx + 1, self.end_col_idx));
            }

            for i in self.start_col_idx..self.end_col_idx {
                indices.push((self.row_idx + 1, i));
            }
        }

        return indices;
    }
}

fn parse_engine_numbers(line: &str, row_idx: usize) -> Vec<EngineNumber> {
    let mut numbers: Vec<EngineNumber> = Vec::new();
    let mut curr_start_idx: usize = 0;
    let mut curr_str_num: String = String::new();
    let mut state: bool = false;
    for (i, c) in line.chars().enumerate() {
        let is_digit = c.is_digit(10);

        if is_digit && !state {
            state = true;
            curr_start_idx = i;
            curr_str_num.clear();
            curr_str_num.push(c);
            continue;
        }

        if is_digit && state {
            curr_str_num.push(c);
            continue;
        }

        if !is_digit && !state {
            continue;
        }

        if (!is_digit) && state {
            state = false;
            numbers.push(EngineNumber {
                row_idx: row_idx,
                start_col_idx: curr_start_idx,
                end_col_idx: i,
                num: curr_str_num
                    .parse::<u32>()
                    .expect("Should be able to parse number"),
            });
            continue;
        }
    }

    if state {
        numbers.push(EngineNumber {
            row_idx: row_idx,
            start_col_idx: curr_start_idx,
            end_col_idx: line.chars().count(),
            num: curr_str_num
                .parse::<u32>()
                .expect("Should be able to parse number"),
        });
    }

    return numbers;
}

fn parse_engine_symbols(line: &str, row_idx: usize) -> HashMap<(usize, usize), char> {
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    for (i, c) in line.chars().enumerate() {
        if !c.is_digit(10) && c != '.' {
            symbols.insert((row_idx, i), c);
        }
    }

    return symbols;
}

fn main() {
    let filename: &str = "../schematic.txt";
    let file_contents: String = read_file(filename);
    let lines: Split<char> = file_contents.split::<char>('\n');

    let mut eng_nums: Vec<EngineNumber> = Vec::new();
    let mut eng_syms: HashMap<(usize, usize), char> = HashMap::new();
    let n_rows: usize = lines.clone().count();
    let mut n_cols: usize = 0;
    for (i, line) in lines.enumerate() {
        n_cols = line.chars().count();
        eng_nums.append(&mut parse_engine_numbers(line, i));
        eng_syms.extend(parse_engine_symbols(line, i));
    }

    let mut part_num_sum: u32 = 0;
    let mut gears: HashMap<(usize, usize), EngineGear> = HashMap::new();
    for eng_num in eng_nums {
        let mut is_part_num: bool = false;
        for surr_indices in eng_num.surrounding_indices(n_rows, n_cols) {
            if let Some(&sym) = eng_syms.get(&surr_indices) {
                is_part_num = true;
                if sym == '*' {
                    if let Some(gear) = gears.get_mut(&surr_indices) {
                        gear.n_part_nums += 1;
                        gear.ratio *= eng_num.num;
                    } else {
                        gears.insert(
                            surr_indices,
                            EngineGear {
                                n_part_nums: 1,
                                ratio: eng_num.num,
                            },
                        );
                    }
                }
                break;
            }
        }

        if is_part_num {
            part_num_sum += eng_num.num;
        }
    }

    let mut valid_gears_ratio_sum: u32 = 0;
    for (_, gear) in gears {
        if gear.n_part_nums == 2 {
            valid_gears_ratio_sum += gear.ratio;
        }
    }

    println!("{}", part_num_sum);
    println!("{}", valid_gears_ratio_sum);
}
