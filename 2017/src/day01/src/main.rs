use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
    let args:Vec<String> = env::args().collect();
    let (filename,) = read_args(&args);
    let contents = read_file(filename);

    let total = sum_list(&string_to_list::<i32>(&contents));
    println!("The result is '{}'\n", total);
}

fn read_args(args: &[String]) -> (&str,) {
    let filename = &args[1];
    (filename,)
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

fn sum_list(list: &Vec<i32>) -> i32 {

    let (total, previous) = list.iter().fold((0, -1), |accumulator, value| {
        let (total, previous) = accumulator;
        (sum(&previous, value, &total), *value)
    });

    sum(&previous, &list[0], &total)
}

fn sum(previous: &i32, value: &i32, total: &i32) -> i32 {
    if previous == value {
        return total + value;
    }

    *total
}

#[test]
fn test_case_one() {
    assert_eq!(sum_list(&vec![1, 1, 2, 2]), 3);
}

#[test]
fn test_case_two() {
    assert_eq!(sum_list(&vec![1, 1, 1, 1]), 4);
}

#[test]
fn test_case_three() {
    assert_eq!(sum_list(&vec![1, 2, 3, 4]), 0);
}

#[test]
fn test_case_four() {
    assert_eq!(sum_list(&vec![9, 1, 2, 1, 2, 1, 2, 9]), 9);
}
