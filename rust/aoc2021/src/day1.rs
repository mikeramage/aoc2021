use crate::utils;

///Day 1 solution
pub fn day1() -> (usize, usize) {
    let measurements: Vec<i32> = utils::parse_input("input/day1.txt");
    (get_count_of_increases(&measurements, 1), get_count_of_increases(&measurements, 3))
}

/// Takes a vector of measurements and a window size and for each
/// sliding window of size window_size it counts the number of times
/// the sum of the values in the current window is greater than at the
/// previous window position. The window slides along by 1 index each time.    
fn get_count_of_increases(measurements: &[i32], window_size: usize) -> usize {
    assert!(
        measurements.len() > window_size,
        "Window size is greater than the number of measurements!"
    );

    // Since B + C + D - (A + B + C) = D - A ...
    let count = measurements
        .iter()
        .zip(&measurements[window_size..]) //truncates larger vector
        .map(|(x, y)| y - x)
        .filter(|x| x > &0)
        .count();

    count
}

///Day 1 solution - imperative
#[allow(dead_code)]
pub fn day1_imp() -> (usize, usize) {
    let measurements = utils::parse_input_as_int_vec_imp("input/day1.txt");

    println!(
        "Part1 - number of measurements greater than previous: {}",
        get_count_of_increases_imp(&measurements, 1)
    );
    println!(
        "Part2 - number of 3-value window sums greater than previous: {}",
        get_count_of_increases_imp(&measurements, 3)
    );

    (get_count_of_increases_imp(&measurements, 1), get_count_of_increases_imp(&measurements, 3))
}

/// Takes a vector of measurements and a window size and for each
/// sliding window of size window_size it counts the number of times
/// the sum of the values in the current window is greater than at the
/// previous window position. The window slides along by 1 index each time.
#[allow(dead_code)]
fn get_count_of_increases_imp(measurements: &[i32], window_size: usize) -> usize {
    let mut count = 0;
    let mut previous_value = 0;
    assert!(
        measurements.len() > window_size,
        "Window size is greater than the number of measurements!"
    );
    for measurement in measurements.iter().take(window_size) {
        previous_value += measurement;
    }

    for i in window_size..measurements.len() {
        let mut current_value = 0;
        for j in 0..window_size {
            current_value += measurements[i - j];
        }

        if current_value > previous_value {
            count += 1;
        }

        previous_value = current_value;
    }

    count
}
