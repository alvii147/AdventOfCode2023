use std::fs;

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

fn parse_seq(line: &str) -> Vec<i64> {
    return line
        .split_whitespace()
        .map(|n| {
            n.parse::<i64>()
                .expect("Should be able to parse sequence numbers")
        })
        .collect::<Vec<i64>>();
}

fn compute_seq_diff(seq: &Vec<i64>) -> Vec<i64> {
    return seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>();
}

fn extrapolate_sequence(seq: &Vec<i64>, backward: bool) -> i64 {
    let mut s: Vec<i64> = seq.clone();
    let extrapolator: fn(&Vec<i64>, u32) -> i64 = if backward {
        |s: &Vec<i64>, i: u32| *s.first().expect("Should have first term") * i64::pow(-1, i)
    } else {
        |s: &Vec<i64>, _i: u32| *s.last().expect("Should have last term")
    };
    let mut i: u32 = 0;
    let mut extrapolated_term: i64 = extrapolator(&s, i);
    loop {
        s = compute_seq_diff(&s);
        if s.iter().all(|&n| n == 0) {
            break;
        }

        i += 1;
        extrapolated_term += extrapolator(&s, i);
    }

    return extrapolated_term;
}

fn main() {
    let filename: &str = "../report.txt";
    let file_contents: String = read_file(filename);

    let mut extrapolated_forward_sum: i64 = 0;
    let mut extrapolated_backward_sum: i64 = 0;
    for line in file_contents.split::<char>('\n') {
        let seq: Vec<i64> = parse_seq(line);
        extrapolated_forward_sum += extrapolate_sequence(&seq, false);
        extrapolated_backward_sum += extrapolate_sequence(&seq, true);
    }

    println!("{}", extrapolated_forward_sum);
    println!("{}", extrapolated_backward_sum);
}
