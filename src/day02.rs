use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(PartialEq, Debug)]
struct Position(i32, i32);

enum Move {
    Forward(i32),
    Up(i32),
    Down(i32)
}

impl From<String> for Move {
    fn from(line: String) -> Move {
        match line.split_once(" ") {
            Some((word, num)) => {
                let displacement = num.parse::<i32>().unwrap();
                match word {
                    "forward" => Move::Forward(displacement),
                    "up" => Move::Up(displacement),
                    "down" => Move::Down(displacement),
                    _ => panic!("Invalid instruction: unknown move")
                }
            },
            None => panic!("Invalid instruction: unsplittable input")
        }
    }
}

fn load_input<P>(filename: P) -> Vec<Move>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let res = BufReader::new(file).lines()
        .map(|line| {
            match line {
                Ok(l) => Move::from(l),
                Err(e) => panic!("{}", e)
            }
            
        }).collect::<Vec<Move>>();
    res
}

fn displace(current: Position, m: Move) -> Position {
    match m {
        Move::Forward(x) => Position(current.0 + x, current.1),
        Move::Up(y) => Position(current.0, current.1 - y),
        Move::Down(y) => Position(current.0, current.1 + y)
    }
}

fn calculate_position(start: Position, moves: Vec<Move>) -> Position {
    let mut pos = start;
    for m in moves {
        pos = displace(pos, m)
    };
    pos
}

pub fn solve(filename: &str) -> i32 {
    let input = load_input(filename);
    let start = Position(0, 0);
    let end = calculate_position(start, input);
    end.0 * end.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displace_ok() {
        assert_eq!(displace(Position(0, 0), Move::Forward(2)), Position(2, 0));
        assert_eq!(displace(Position(0, 0), Move::Down(2)), Position(0, 2));
        assert_eq!(displace(Position(0, 0), Move::Up(2)), Position(0, -2));
    }
    #[test]
    fn calulate_position_ok() {
        let moves = vec![
            Move::Forward(2),
            Move::Down(2),
            Move::Up(1)
        ];
        let start = Position(0, 0);
        assert_eq!(calculate_position(start, moves), Position(2, 1));
    }

    #[test]
    fn solve_ok() {
        let res = solve("inputs/day02-1-test.txt");
        assert_eq!(res, 150);
    }
}