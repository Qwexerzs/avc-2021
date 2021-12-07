mod solutions;

use solutions::*;
use std::fs;

fn main() {
    day1();
}

fn day1() {
    println!(
        "{}",
        day_1::puzzle_1(read_file(String::from("input/day_1.txt")))
    );
}

fn read_file(file_name: String) -> String {
    fs::read_to_string(file_name).expect("Could Not Read Input")
}
