use std::fs;
use aoc_2::{part_1, part_2};

fn main() {
    run_part_1();
    run_part_2();
}

fn run_part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output = part_1(&input);
    println!("part 1: {}", output);
}

fn run_part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output = part_2(&input);
    println!("part 2: {}", output);
}
