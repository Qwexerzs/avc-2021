mod solutions;

use solutions::*;
use std::fs;

fn main() {
    println!("Day 1 Puzzle 1 Output: {}", day1_puzzle1());
    println!("Day 1 Puzzle 2 Output: {}", day1_puzzle2());
}

fn day1_puzzle1() -> i32 {
    day_1::puzzle_1(read_file(String::from("input/day_1_puzzle_1.txt")))
}

fn day1_puzzle2() -> i32 {
    day_1::puzzle_2(read_file(String::from("input/day_1_puzzle_2.txt")))
}

fn read_file(file_name: String) -> String {
    fs::read_to_string(file_name).expect("Could Not Read Input")
}
