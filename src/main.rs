use std::fs;
use aoc_2::part_1;

fn main() {
    run_part_1();
}

fn run_part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output = part_1(&input);
    println!("part 1: {}", output);
}
