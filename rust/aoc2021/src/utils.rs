use std::fmt::Debug;
use std::fs;
use std::str;

/// Takes a file containing a list of strings, one per line, and returns the
/// list as a vector of T
pub fn parse_input<T: str::FromStr>(input_file: &str) -> Vec<T>
where
    <T as str::FromStr>::Err: Debug,
{
    let input = fs::read_to_string(input_file).expect("Oh dear, couldn't read file!");
    let vector: Vec<T> = input.lines().map(|line| line.parse().unwrap()).collect();
    vector
}

/// Takes a file containing a list of integers, one per line, and returns the
/// list of integers as a Vec<i32> - imperative style
#[allow(dead_code)]
pub fn parse_input_as_int_vec_imp(input_file: &str) -> Vec<i32> {
    let input = fs::read_to_string(input_file).expect("Oh dear, couldn't read file!");
    let mut vector: Vec<i32> = vec![];
    for line in input.lines() {
        vector.push(line.parse::<i32>().expect("Failed to parse line!"));
    }
    vector
}
