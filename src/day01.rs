use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Vec<usize>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let res = io::BufReader::new(file).lines()
        .map(|line| {
            match line {
                Ok(line) => line.parse::<usize>().unwrap(),
                Err(e) => panic!("{}", e)
            }
            
        }).collect::<Vec<usize>>();

    Ok(res)
}

fn count_increases(measurements: Vec<usize>) -> usize {
    let mut prev = measurements.clone();
    prev.insert(0, measurements[0]);
    let (_, prev ) = prev.split_last().unwrap();
    let increases = measurements
        .iter()
        .zip(prev)
        .filter(|(current, previous)| {
            current > previous
        })
        .collect::<Vec<(&usize, &usize)>>();
    increases.len()
}

fn windowed_sum(measurements: Vec<usize>) -> Vec<usize> {
    (0..measurements.len() - 2).map(|i| {
        measurements[i] + measurements[i + 1] + measurements[i + 2]
    }).collect::<Vec<usize>>()

}

pub fn solve(filename: &str) -> usize {
    let input = read_lines(filename).unwrap();
    count_increases(input)
}

pub fn solve_with_windows(filename: &str) -> usize {
    let input = read_lines(filename).unwrap();
    let windowed = windowed_sum(input);
    count_increases(windowed)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_increases_ok() {
        let res = count_increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(res, 7);
    }
    #[test]
    fn solve_ok() {
        let res = solve("inputs/day01-1-test.txt");
        assert_eq!(res, 7);
    }
    #[test]
    fn windowed_sum_ok() {
        let res = windowed_sum(vec![1, 2, 3, 4, 5]);
        assert_eq!(res, vec![6, 9, 12]);
    }

    #[test]
    fn solve_with_window_ok() {
        let res = solve_with_windows("inputs/day01-1-test.txt");
        assert_eq!(res, 5);
    }

}
