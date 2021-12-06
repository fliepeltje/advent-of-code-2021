mod day01;
mod day02;
fn main() {
    println!("Day 1 part 1: {}", day01::solve("inputs/day01-1.txt"));
    println!("Day 1 part 2: {}", day01::solve_with_windows("inputs/day01-2.txt"));
    println!("Day 2 part 1: {}", day02::solve("inputs/day02-1.txt"));
}
