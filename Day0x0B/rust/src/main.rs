use std::fs;

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

fn get_empty_row_indices(lines: &Vec<String>) -> Vec<usize> {
    return lines
        .iter()
        .enumerate()
        .filter(|&(_, l)| !l.contains('#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
}

fn get_empty_col_indices(lines: &Vec<String>) -> Vec<usize> {
    let n_rows: usize = lines.len();
    let mut n_cols: usize = 0;
    for line in lines.iter() {
        n_cols = line.len();
        break;
    }

    let mut indices: Vec<usize> = Vec::new();
    for col in 0..n_cols {
        let mut is_empty: bool = true;
        for row in 0..n_rows {
            if lines.get(row).unwrap().chars().nth(col).unwrap() == '#' {
                is_empty = false;
                break;
            }
        }

        if !is_empty {
            continue;
        }

        indices.push(col);
    }

    return indices;
}

fn parse_galaxies(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        line.match_indices('#')
            .for_each(|(col, _)| galaxies.push((row, col)));
    }

    return galaxies;
}

fn compute_distances_sum(
    galaxies: &Vec<(usize, usize)>,
    expansion_factor: usize,
    empty_row_indices: &Vec<usize>,
    empty_col_indices: &Vec<usize>,
) -> u64 {
    let mut galaxies_clone: Vec<(usize, usize)> = galaxies.clone();

    let mut offset: usize = 0;
    for row in empty_row_indices {
        galaxies_clone.iter_mut().for_each(|g| {
            if g.0 > row + offset {
                g.0 += expansion_factor - 1;
            };
        });
        offset += expansion_factor - 1;
    }

    offset = 0;
    for col in empty_col_indices {
        galaxies_clone.iter_mut().for_each(|g| {
            if g.1 > col + offset {
                g.1 += expansion_factor - 1;
            };
        });
        offset += expansion_factor - 1;
    }

    let mut distances_sum: u64 = 0;
    for i in 0..galaxies_clone.len() {
        for j in i + 1..galaxies_clone.len() {
            let galaxy1 = galaxies_clone[i];
            let galaxy2 = galaxies_clone[j];
            let distance = u64::abs_diff(galaxy1.0 as u64, galaxy2.0 as u64)
                + u64::abs_diff(galaxy1.1 as u64, galaxy2.1 as u64);
            distances_sum += distance;
        }
    }

    return distances_sum;
}

fn main() {
    let filename: &str = "../galaxies.txt";
    let file_contents: String = read_file(filename);
    let lines: Vec<String> = file_contents
        .split::<char>('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let empty_row_indices: Vec<usize> = get_empty_row_indices(&lines);
    let empty_col_indices: Vec<usize> = get_empty_col_indices(&lines);
    let galaxies: Vec<(usize, usize)> = parse_galaxies(&lines);

    let expansion_factor: usize = 2;
    let distances_sum = compute_distances_sum(
        &galaxies,
        expansion_factor,
        &empty_row_indices,
        &empty_col_indices,
    );

    println!("{}", distances_sum);

    let expansion_factor: usize = 1000000;
    let distances_sum = compute_distances_sum(
        &galaxies,
        expansion_factor,
        &empty_row_indices,
        &empty_col_indices,
    );

    println!("{}", distances_sum);
}
