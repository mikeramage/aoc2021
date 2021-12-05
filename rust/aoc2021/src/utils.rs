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

pub fn parse_input_by_blank_lines(input_file: &str) -> Vec<String> {
    let input = fs::read_to_string(input_file).expect("Oh dear, couldn't read file!");
    let vector: Vec<String> = input.split("\n\n").map(|x| x.to_string()).collect();
    vector
}

/// Converts a string representing a binary number of up to 63 characters, e.g. "0100110110111" and converts to a usize.
///
/// It works like this:
///  - Converts the string to a Chars iterator and enumerates - so if you've got [(0, '0'), (1, '1'), (2, '0')...]
///  - Maps these values by left shifting the appropriate amount: ((length of the string - 1) - index) in the enumeration.
/// To see this, remember you want to left shift the last index by 0.
///  - Sums them.
pub fn string_binary_to_usize(binary_as_string: String) -> usize {
    assert!(binary_as_string.len() < 64); //Obviously this breaks if usize != 64 but I don't care.
    let binary_as_chars = binary_as_string.chars();
    let shift = binary_as_string.len() - 1;
    let number: usize = binary_as_chars
        .enumerate()
        .map(|(index, digit)| {
            if let '1' = digit {
                1 << (shift - index)
            } else {
                0
            }
        })
        .sum();
    number
}
