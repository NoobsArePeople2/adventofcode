use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

// --- Day 1: Inverse Captcha ---
// The night before Christmas, one of Santa's Elves calls you in a panic. 
// "The printer's broken! We can't print the Naughty or Nice List!" By the time 
// you make it to sub-basement 17, there are only a few minutes until midnight. 
// "We have a big problem," she says; "there must be almost fifty bugs in this 
// system, but nothing else can print The List. Stand in this square, quick! 
// There's no time to explain; if you can convince them to pay you in stars, 
// you'll be able to--" She pulls a lever and the world goes blurry.
//
// When your eyes can focus again, everything seems a lot more pixelated than 
// before. She must have sent you inside the computer! You check the system clock: 
// 25 milliseconds until midnight. With that much time, you should be able to 
// collect all fifty stars by December 25th.
// 
// Collect stars by solving puzzles. Two puzzles will be made available on each 
// day millisecond in the advent calendar; the second puzzle is unlocked when 
// you complete the first. Each puzzle grants one star. Good luck!
//
// You're standing in a room with "digitization quarantine" written in LEDs 
// along one wall. The only door is locked, but it includes a small interface. 
// "Restricted Area - Strictly No Digitized Users Allowed."
//
// It goes on to explain that you may only leave by solving a captcha to prove 
// you're not a human. Apparently, you only get one millisecond to solve the captcha: 
// too fast for a normal human, but it feels like hours to you.
//
// The captcha requires you to review a sequence of digits (your puzzle input) 
// and find the sum of all digits that match the next digit in the list. The 
// list is circular, so the digit after the last digit is the first digit in the list.
//
// For example:
//
//    - 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches 
//      the second digit and the third digit (2) matches the fourth digit.
//    - 1111 produces 4 because each digit (all 1) matches the next.
//    - 1234 produces 0 because no digit matches the next.
//    - 91212129 produces 9 because the only digit that matches the next one 
//      is the last digit, 9.
//
// --- Part Two ---
//
// You notice a progress bar that jumps to 50% completion. Apparently, the door isn't 
// yet satisfied, but it did emit a star as encouragement. The instructions change:
//
// Now, instead of considering the next digit, it wants you to consider the digit halfway 
// around the circular list. That is, if your list contains 10 items, only include a digit 
// in your sum if the digit 10/2 = 5 steps forward matches it. Fortunately, your list has 
// an even number of elements.
//
// For example:
//
//    - 1212 produces 6: the list contains 4 items, and all four digits match the 
//      digit 2 items ahead.
//    - 1221 produces 0, because every comparison is between a 1 and a 2.
//    - 123425 produces 4, because both 2s match each other, but no other digit 
//      has a match.
//    - 123123 produces 12.
//    - 12131415 produces 4.

fn main() {
    let args:Vec<String> = env::args().collect();
    let (part, filename) = read_args(&args);
    let contents = read_file(filename);

    let mut list = string_to_list::<i32>(&contents);
    if part == "one" {
        let total = sum_list(&mut list, 1);
        println!("The result for part one is '{}'\n", total);
    } else if part == "two" {
        let step = list.len() / 2;
        let total = sum_list(&mut list, step);
        println!("The result for part two is '{}'\n", total);
    }
}

fn read_args(args: &[String]) -> (&str, &str) {
    let part = &args[1];
    let filename = &args[2];
    (part, filename)
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    println!("Input text: '{}'\n", &contents);

    return contents
}

fn string_to_list<T>(s: &str) -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
    let nums: Vec<T> = s.chars()
        .map(|s| s.to_string().parse::<T>().unwrap())
        .collect();

    nums
}

#[test]
fn test_string_to_list() {
    assert_eq!(string_to_list::<i32>("1122"), [1, 1, 2, 2]);
    assert_eq!(string_to_list::<i32>("1111"), [1, 1, 1, 1]);
    assert_eq!(string_to_list::<i32>("1234"), [1, 2, 3, 4]);
    assert_eq!(string_to_list::<i32>("91212129"), [9, 1, 2, 1, 2, 1, 2, 9]);
}

fn sum_list(list: &mut Vec<i32>, step: usize) -> i32 {

    let mut sum = 0;
    for (i, value) in list.iter().enumerate() {
        if values_match(&value, &list[(i + step) % list.len()]) {
            sum += value;
        }
    }

    sum
}

fn values_match(a: &i32, b: &i32) -> bool {
    a == b
}

#[test]
fn test_part_one_case_one() {
    assert_eq!(sum_list(&mut vec![1, 1, 2, 2], 1), 3);
}

#[test]
fn test_part_one_case_two() {
    assert_eq!(sum_list(&mut vec![1, 1, 1, 1], 1), 4);
}

#[test]
fn test_part_one_case_three() {
    assert_eq!(sum_list(&mut vec![1, 2, 3, 4], 1), 0);
}

#[test]
fn test_part_one_case_four() {
    assert_eq!(sum_list(&mut vec![9, 1, 2, 1, 2, 1, 2, 9], 1), 9);
}

#[test]
fn test_part_two_case_one() {
    let mut list = vec![1, 2, 1, 2];
    let step = list.len() / 2;
    assert_eq!(sum_list(&mut list, step), 6);
}

#[test]
fn test_part_two_case_two() {
    let mut list = vec![1, 2, 2, 1];
    let step = list.len() / 2;
    assert_eq!(sum_list(&mut list, step), 0);
}

#[test]
fn test_part_two_case_three() {
    let mut list = vec![1, 2, 3, 4, 2, 5];
    let step = list.len() / 2;
    assert_eq!(sum_list(&mut list, step), 4);
}

#[test]
fn test_part_two_case_four() {
    let mut list = vec![1, 2, 3, 1, 2, 3];
    let step = list.len() / 2;
    assert_eq!(sum_list(&mut list, step), 12);
}

#[test]
fn test_part_two_case_five() {
    let mut list = vec![1, 2, 1, 3, 1, 4, 1, 5];
    let step = list.len() / 2;
    assert_eq!(sum_list(&mut list, step), 4);
}
