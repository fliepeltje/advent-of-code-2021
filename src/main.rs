mod day01;

fn main() {
    println!("Day 1 part 1: {}", day01::solve("inputs/day01-1.txt"));
    println!("Day 1 part 2: {}", day01::solve_with_windows("inputs/day01-2.txt"))
}
